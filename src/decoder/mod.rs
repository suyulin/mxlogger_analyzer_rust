use aes::cipher::AsyncStreamCipher;
use byteorder::{ByteOrder, LittleEndian};

use crate::{crypto, log_serialize_generated::log_serialize, logger::Level};

const LENGTH_PREFIX_BYTES: usize = 4;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogRecord {
    pub timestamp_micros: u64,
    pub level: String,
    pub name: String,
    pub tag: String,
    pub thread_id: i32,
    pub is_main_thread: bool,
    pub message: String,
    pub source_file: String,
    pub source_offset: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DecodeStats {
    pub decoded_records: usize,
    pub damaged_records: usize,
    pub truncated_tail: bool,
    pub empty_file: bool,
}

#[derive(Clone, Debug, Default)]
pub struct DecodeResult {
    pub records: Vec<LogRecord>,
    pub stats: DecodeStats,
}

pub fn decode_bytes(
    binary_data: &[u8],
    crypt_key: Option<&str>,
    iv: Option<&str>,
    source_file: &str,
) -> Result<DecodeResult, String> {
    if binary_data.is_empty() {
        return Ok(DecodeResult {
            stats: DecodeStats {
                empty_file: true,
                ..DecodeStats::default()
            },
            ..DecodeResult::default()
        });
    }
    if binary_data.len() < LENGTH_PREFIX_BYTES {
        return Err(format!(
            "{source_file}: file is shorter than the MxLogger header"
        ));
    }

    let declared_size = LittleEndian::read_u32(&binary_data[..LENGTH_PREFIX_BYTES]) as usize;
    if declared_size < LENGTH_PREFIX_BYTES {
        return Err(format!("{source_file}: invalid declared file size"));
    }

    let mut result = DecodeResult::default();
    let limit = declared_size.min(binary_data.len());
    if declared_size > binary_data.len() {
        result.stats.truncated_tail = true;
    }
    let decryptor = match (crypt_key, iv) {
        (Some(key), Some(iv)) => Some(crypto::create_decryptor(key, iv)),
        (None, None) => None,
        _ => return Err("both MXLOGGER_CRYPT_KEY and MXLOGGER_IV_KEY are required".into()),
    };

    let mut begin = LENGTH_PREFIX_BYTES;
    while begin < limit {
        if begin + LENGTH_PREFIX_BYTES > limit {
            result.stats.truncated_tail = true;
            break;
        }
        let item_size =
            LittleEndian::read_u32(&binary_data[begin..begin + LENGTH_PREFIX_BYTES]) as usize;
        let payload_start = begin + LENGTH_PREFIX_BYTES;
        let Some(payload_end) = payload_start.checked_add(item_size) else {
            result.stats.damaged_records += 1;
            break;
        };
        if item_size == 0 {
            result.stats.damaged_records += 1;
            begin += LENGTH_PREFIX_BYTES;
            continue;
        }
        if payload_end > limit {
            result.stats.truncated_tail = true;
            break;
        }

        let mut payload = binary_data[payload_start..payload_end].to_vec();
        if let Some(cipher) = decryptor.clone() {
            cipher.decrypt(&mut payload);
        }
        match flatbuffers::root::<log_serialize>(&payload) {
            Ok(log) => {
                let level = Level::try_from(log.level() as i32)
                    .unwrap_or(Level::Info)
                    .name()
                    .to_owned();
                result.records.push(LogRecord {
                    timestamp_micros: log.timestamp(),
                    level,
                    name: log.name().unwrap_or_default().to_owned(),
                    tag: log.tag().unwrap_or_default().to_owned(),
                    thread_id: log.thread_id(),
                    is_main_thread: log.is_main_thread() != 0,
                    message: log.msg().unwrap_or_default().to_owned(),
                    source_file: source_file.to_owned(),
                    source_offset: begin,
                });
            }
            Err(_) => result.stats.damaged_records += 1,
        }
        begin = payload_end;
    }
    result.stats.decoded_records = result.records.len();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes::{cipher::KeyIvInit, Aes128};
    use cfb_mode::Encryptor;
    use flatbuffers::FlatBufferBuilder;

    use crate::log_serialize_generated::{log_serializeArgs, log_serializeBuilder};

    fn encoded_file(records: &[(u64, &str)], encrypted: bool) -> Vec<u8> {
        let mut file = vec![0; LENGTH_PREFIX_BYTES];
        for (timestamp, message) in records {
            let mut builder = FlatBufferBuilder::new();
            let name = builder.create_string("APP");
            let tag = builder.create_string("test");
            let msg = builder.create_string(message);
            let args = log_serializeArgs {
                name: Some(name),
                tag: Some(tag),
                msg: Some(msg),
                level: 1,
                thread_id: 7,
                is_main_thread: 1,
                timestamp: *timestamp,
            };
            let mut record_builder = log_serializeBuilder::new(&mut builder);
            record_builder.add_name(args.name.unwrap());
            record_builder.add_tag(args.tag.unwrap());
            record_builder.add_msg(args.msg.unwrap());
            record_builder.add_level(args.level);
            record_builder.add_thread_id(args.thread_id);
            record_builder.add_is_main_thread(args.is_main_thread);
            record_builder.add_timestamp(args.timestamp);
            let record = record_builder.finish();
            builder.finish(record, None);
            let mut payload = builder.finished_data().to_vec();
            if encrypted {
                let key = crypto::replenish_byte("test-key");
                let iv = crypto::replenish_byte("test-iv");
                Encryptor::<Aes128>::new(&key.into(), &iv.into()).encrypt(&mut payload);
            }
            file.extend_from_slice(&(payload.len() as u32).to_le_bytes());
            file.extend_from_slice(&payload);
        }
        let size = file.len() as u32;
        file[..LENGTH_PREFIX_BYTES].copy_from_slice(&size.to_le_bytes());
        file
    }

    #[test]
    fn decodes_plain_records() {
        let input = encoded_file(&[(2, "second"), (1, "first")], false);
        let decoded = decode_bytes(&input, None, None, "app.mx").unwrap();

        assert_eq!(decoded.records.len(), 2);
        assert_eq!(decoded.records[0].message, "second");
        assert_eq!(decoded.records[0].name, "APP");
        assert!(decoded.records[0].is_main_thread);
    }

    #[test]
    fn decodes_encrypted_records() {
        let input = encoded_file(&[(1, "encrypted")], true);
        let decoded = decode_bytes(&input, Some("test-key"), Some("test-iv"), "app.mx").unwrap();

        assert_eq!(decoded.records[0].message, "encrypted");
    }

    #[test]
    fn tolerates_truncated_final_record() {
        let mut input = encoded_file(&[(1, "first"), (2, "second")], false);
        input.truncate(input.len() - 5);
        let declared = (input.len() + 5) as u32;
        input[..LENGTH_PREFIX_BYTES].copy_from_slice(&declared.to_le_bytes());

        let decoded = decode_bytes(&input, None, None, "active.mx").unwrap();

        assert_eq!(decoded.records.len(), 1);
        assert!(decoded.stats.truncated_tail);
    }

    #[test]
    fn marks_empty_file_without_failing() {
        let decoded = decode_bytes(&[], None, None, "empty.mx").unwrap();
        assert!(decoded.stats.empty_file);
        assert!(decoded.records.is_empty());
    }
}
