use byteorder::{LittleEndian, ReadBytesExt};
use tokio::{fs::File, io::AsyncWriteExt};
use crate::{crypto, log_serialize_generated::log_serialize, logger::Level};
use aes::cipher::AsyncStreamCipher;
use chrono_tz::Tz;

pub async fn decode(
    binary_data: &[u8],
    crypt_key: Option<&str>,
    iv: Option<&str>,
    output_filename: &str,
    timezone: Option<&str>,
    on_error: impl Fn(i32, String),
    _on_repeat_error: impl Fn(),
    _total_callback: impl Fn(i32),
    progress_callback: impl Fn(i32, i32),
) -> Result<(), Box<dyn std::error::Error>> {
    let offset = 4;
    let mut error_count = 0;

    let aes128_cfb_dec = match (crypt_key, iv) {
        (Some(key), Some(iv)) => Some(crypto::create_decryptor(key, iv)),
        _ => None,
    };

    let mut reader = &binary_data[..offset];
    let total_size = reader.read_u32::<LittleEndian>()? as usize;
    
    let file_path = format!("{}.log", output_filename);
    let mut file = File::create(&file_path).await?;

    let mut begin = offset;
    while begin < binary_data.len() && begin < total_size {
        // 确保有足够的字节读取item_size
        if begin + offset > binary_data.len() {
            break;
        }
        
        let mut reader = &binary_data[begin..begin+offset];
        let item_size = reader.read_u32::<LittleEndian>().unwrap_or(0) as usize;
        let start = begin + offset;
        
        // 确保不会读取超出数组边界
        if start + item_size > binary_data.len() {
            error_count += 1;
            on_error(error_count, "Item size exceeds buffer length".to_string());
            break;
        }
        
        let buffer = &binary_data[start..start+item_size];
        let mut decrypted_data = buffer.to_vec();
        
        // 安全地处理解密
        if let Some(decryptor) = aes128_cfb_dec.clone() {
            decryptor.decrypt(&mut decrypted_data);
        }
    
        // Deserialize and process your data here using logDeserialize (pseudo-code)
        if let Some(log_serialize) = deserialize_log(&decrypted_data) {
            let timestamp = log_serialize.timestamp();
            
            // 简化的时区处理逻辑
            let time = format_timestamp_with_timezone(timestamp, timezone);
            
            let msg = log_serialize.msg().unwrap_or("no message");
            let level = log_serialize.level();
            let level = Level::try_from(level as i32).unwrap_or(Level::INFO);
            let log_serialize_str = format!("{} {} {}\n", time, level.name(), msg);
            
            if let Err(e) = file.write_all(log_serialize_str.as_bytes()).await {
                error_count += 1;
                on_error(error_count, format!("Failed to write to file: {}", e));
            }
        } else {
            error_count += 1;
            on_error(error_count, "Binary file parse failed".to_string());
        }

        progress_callback(total_size as i32, begin as i32);
        begin += offset + item_size;
    }

    Ok(())
}

fn deserialize_log(bytes: &[u8]) -> Option<log_serialize<'_>> {
    flatbuffers::root::<log_serialize>(bytes).ok()
}

fn format_timestamp_with_timezone(timestamp: u64, timezone: Option<&str>) -> String {
    let utc_time = match chrono::DateTime::<chrono::Utc>::from_timestamp_micros(timestamp as i64) {
        Some(time) => time,
        None => {
            // 如果时间戳解析失败，返回错误信息
            return "Invalid timestamp".to_string();
        }
    };
    
    let tz = timezone
        .and_then(|tz_str| tz_str.parse::<Tz>().ok())
        .unwrap_or_else(|| "Asia/Shanghai".parse::<Tz>().unwrap());
    
    utc_time.with_timezone(&tz).format("%Y-%m-%d %H:%M:%S").to_string()
} 