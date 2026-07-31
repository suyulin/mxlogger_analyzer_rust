#![allow(mismatched_lifetime_syntaxes)]

mod crypto;
mod decoder;
#[rustfmt::skip]
mod log_serialize_generated;
mod logger;

use std::{
    fs,
    io::{self, Cursor, Read, Write},
    path::{Path, PathBuf},
};

use chrono::{DateTime, SecondsFormat, Utc};
use chrono_tz::Tz;
use clap::{ArgAction, Parser};
use decoder::{DecodeStats, LogRecord};
use serde::Serialize;
use zip::ZipArchive;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Decode MxLogger files, directories, or decrypted ZIP bundles",
    after_help = "Encryption is enabled only when both MXLOGGER_CRYPT_KEY and MXLOGGER_IV_KEY are set."
)]
struct Cli {
    /// MxLogger file, directory, or decrypted ZIP bundle
    input: PathBuf,

    /// IANA timezone used in output timestamps
    #[arg(short = 'z', long, default_value = "Asia/Shanghai")]
    timezone: String,

    /// Output path without the .log/.jsonl extension
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Also write structured JSON Lines output
    #[arg(long, action = ArgAction::SetTrue)]
    jsonl: bool,

    /// Show per-file decoding statistics
    #[arg(short, long, action = ArgAction::SetTrue)]
    verbose: bool,
}

#[derive(Debug)]
struct InputLog {
    source_file: String,
    bytes: Vec<u8>,
}

#[derive(Serialize)]
struct JsonLogRecord<'a> {
    timestamp: String,
    level: &'a str,
    name: &'a str,
    tag: &'a str,
    thread_id: i32,
    is_main_thread: bool,
    message: &'a str,
    source_file: &'a str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let timezone = cli.timezone.parse::<Tz>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid IANA timezone: {}", cli.timezone),
        )
    })?;
    let crypt_key = std::env::var("MXLOGGER_CRYPT_KEY").ok();
    let iv_key = std::env::var("MXLOGGER_IV_KEY").ok();
    if crypt_key.is_some() != iv_key.is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "MXLOGGER_CRYPT_KEY and MXLOGGER_IV_KEY must be set together",
        )
        .into());
    }

    let inputs = collect_inputs(&cli.input)?;
    if inputs.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "no MxLogger files were found").into());
    }

    let mut records = Vec::new();
    let mut aggregate = DecodeStats::default();
    let mut fatal_errors = Vec::new();
    for input in inputs {
        match decoder::decode_bytes(
            &input.bytes,
            crypt_key.as_deref(),
            iv_key.as_deref(),
            &input.source_file,
        ) {
            Ok(decoded) => {
                if decoded.records.is_empty() && decoded.stats.damaged_records > 0 {
                    fatal_errors.push(format!(
                        "{}: no records decoded; check the file format or encryption keys",
                        input.source_file
                    ));
                }
                if cli.verbose {
                    eprintln!(
                        "{}: records={} damaged={} truncated_tail={} empty={}",
                        input.source_file,
                        decoded.stats.decoded_records,
                        decoded.stats.damaged_records,
                        decoded.stats.truncated_tail,
                        decoded.stats.empty_file
                    );
                }
                aggregate.decoded_records += decoded.stats.decoded_records;
                aggregate.damaged_records += decoded.stats.damaged_records;
                aggregate.truncated_tail |= decoded.stats.truncated_tail;
                aggregate.empty_file |= decoded.stats.empty_file;
                records.extend(decoded.records);
            }
            Err(error) => fatal_errors.push(error),
        }
    }

    sort_records(&mut records);

    let output_base = cli
        .output
        .unwrap_or_else(|| default_output_base(&cli.input));
    let log_path = append_extension(&output_base, "log");
    write_text_log(&log_path, &records, timezone)?;
    if cli.jsonl {
        write_jsonl(&append_extension(&output_base, "jsonl"), &records, timezone)?;
    }

    eprintln!(
        "decoded={} damaged={} truncated_files={} output={}",
        aggregate.decoded_records,
        aggregate.damaged_records,
        usize::from(aggregate.truncated_tail),
        log_path.display()
    );
    if !fatal_errors.is_empty() {
        for error in &fatal_errors {
            eprintln!("error: {error}");
        }
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} input file(s) could not be decoded", fatal_errors.len()),
        )
        .into());
    }
    Ok(())
}

fn sort_records(records: &mut [LogRecord]) {
    records.sort_by(|left, right| {
        left.timestamp_micros
            .cmp(&right.timestamp_micros)
            .then_with(|| left.source_file.cmp(&right.source_file))
            .then_with(|| left.source_offset.cmp(&right.source_offset))
    });
}

fn collect_inputs(input: &Path) -> io::Result<Vec<InputLog>> {
    if input.is_dir() {
        let mut files = Vec::new();
        collect_directory(input, input, &mut files)?;
        files.sort_by(|left, right| left.source_file.cmp(&right.source_file));
        return Ok(files);
    }
    if !input.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("input does not exist: {}", input.display()),
        ));
    }
    if input
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("zip"))
    {
        return collect_zip(input);
    }
    Ok(vec![InputLog {
        source_file: input
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("input")
            .to_owned(),
        bytes: fs::read(input)?,
    }])
}

fn collect_directory(root: &Path, directory: &Path, files: &mut Vec<InputLog>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_directory(root, &path, files)?;
        } else if is_mxlogger_file(&path) {
            let relative = path.strip_prefix(root).unwrap_or(&path);
            files.push(InputLog {
                source_file: normalized_path(relative),
                bytes: fs::read(&path)?,
            });
        }
    }
    Ok(())
}

fn collect_zip(input: &Path) -> io::Result<Vec<InputLog>> {
    let bytes = fs::read(input)?;
    let mut archive = ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let mut files = Vec::new();
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        if entry.is_dir() || !is_mxlogger_archive_entry(entry.name()) {
            continue;
        }
        let mut data = Vec::with_capacity(entry.size() as usize);
        entry.read_to_end(&mut data)?;
        files.push(InputLog {
            source_file: entry.name().replace('\\', "/"),
            bytes: data,
        });
    }
    files.sort_by(|left, right| left.source_file.cmp(&right.source_file));
    Ok(files)
}

fn is_mxlogger_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mx"))
}

fn is_mxlogger_archive_entry(name: &str) -> bool {
    let normalized = name.replace('\\', "/").to_lowercase();
    normalized.ends_with(".mx") || normalized.contains("/mxlogger/")
}

fn normalized_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn default_output_base(input: &Path) -> PathBuf {
    let stem = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("decoded");
    PathBuf::from(stem)
}

fn append_extension(base: &Path, extension: &str) -> PathBuf {
    PathBuf::from(format!("{}.{}", base.to_string_lossy(), extension))
}

fn format_timestamp(timestamp_micros: u64, timezone: Tz) -> io::Result<DateTime<Tz>> {
    let utc = DateTime::<Utc>::from_timestamp_micros(timestamp_micros as i64).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid timestamp: {timestamp_micros}"),
        )
    })?;
    Ok(utc.with_timezone(&timezone))
}

fn write_text_log(path: &Path, records: &[LogRecord], timezone: Tz) -> io::Result<()> {
    let mut output = fs::File::create(path)?;
    for record in records {
        let timestamp = format_timestamp(record.timestamp_micros, timezone)?;
        let message = record.message.replace('\r', "\\r").replace('\n', "\\n");
        writeln!(
            output,
            "{} {} name={} tag={} thread_id={} main_thread={} source={} {}",
            timestamp.format("%Y-%m-%d %H:%M:%S%.6f%:z"),
            record.level,
            record.name,
            record.tag,
            record.thread_id,
            record.is_main_thread,
            record.source_file,
            message
        )?;
    }
    Ok(())
}

fn write_jsonl(path: &Path, records: &[LogRecord], timezone: Tz) -> io::Result<()> {
    let mut output = fs::File::create(path)?;
    for record in records {
        let timestamp = format_timestamp(record.timestamp_micros, timezone)?
            .to_rfc3339_opts(SecondsFormat::Micros, true);
        serde_json::to_writer(
            &mut output,
            &JsonLogRecord {
                timestamp,
                level: &record.level,
                name: &record.name,
                tag: &record.tag,
                thread_id: record.thread_id,
                is_main_thread: record.is_main_thread,
                message: &record.message,
                source_file: &record.source_file,
            },
        )?;
        output.write_all(b"\n")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    use zip::{write::SimpleFileOptions, ZipWriter};

    fn temporary_path(extension: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "mxlogger_analyzer_test_{}_{}.{}",
            std::process::id(),
            nonce,
            extension
        ))
    }

    #[test]
    fn recognizes_only_mxlogger_entries_in_export_bundle() {
        assert!(is_mxlogger_archive_entry("logs/mxlogger/2026-07-31_app.mx"));
        assert!(is_mxlogger_archive_entry("other/app.mx"));
        assert!(!is_mxlogger_archive_entry("metadata.json"));
        assert!(!is_mxlogger_archive_entry("logs/legacy/app.log"));
    }

    #[test]
    fn output_extension_is_appended_instead_of_replaced() {
        assert_eq!(
            append_extension(Path::new("session.decoded"), "log"),
            PathBuf::from("session.decoded.log")
        );
    }

    #[test]
    fn zip_collection_ignores_metadata_and_legacy_logs() {
        let zip_path = temporary_path("zip");
        let file = fs::File::create(&zip_path).unwrap();
        let mut writer = ZipWriter::new(file);
        writer
            .start_file("metadata.json", SimpleFileOptions::default())
            .unwrap();
        writer.write_all(b"{}").unwrap();
        writer
            .start_file("logs/legacy/app.log", SimpleFileOptions::default())
            .unwrap();
        writer.write_all(b"legacy").unwrap();
        writer
            .start_file(
                "logs/mxlogger/2026-07-31_app.mx",
                SimpleFileOptions::default(),
            )
            .unwrap();
        writer.write_all(&[]).unwrap();
        writer.finish().unwrap();

        let inputs = collect_zip(&zip_path).unwrap();
        fs::remove_file(&zip_path).unwrap();

        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0].source_file, "logs/mxlogger/2026-07-31_app.mx");
    }

    #[test]
    fn global_sort_uses_timestamp_file_and_offset() {
        let mut records = vec![
            record(2, "b.mx", 4),
            record(1, "b.mx", 8),
            record(1, "a.mx", 12),
            record(1, "a.mx", 4),
        ];

        sort_records(&mut records);

        let keys = records
            .iter()
            .map(|record| {
                (
                    record.timestamp_micros,
                    record.source_file.as_str(),
                    record.source_offset,
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            keys,
            vec![
                (1, "a.mx", 4),
                (1, "a.mx", 12),
                (1, "b.mx", 8),
                (2, "b.mx", 4),
            ]
        );
    }

    #[test]
    fn jsonl_contains_the_stable_public_fields() {
        let output = temporary_path("jsonl");
        write_jsonl(
            &output,
            &[record(1_000_000, "app.mx", 4)],
            "UTC".parse().unwrap(),
        )
        .unwrap();
        let value: serde_json::Value =
            serde_json::from_str(fs::read_to_string(&output).unwrap().trim()).unwrap();
        fs::remove_file(&output).unwrap();

        let object = value.as_object().unwrap();
        assert_eq!(
            object
                .keys()
                .cloned()
                .collect::<std::collections::BTreeSet<_>>(),
            [
                "is_main_thread",
                "level",
                "message",
                "name",
                "source_file",
                "tag",
                "thread_id",
                "timestamp",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect()
        );
    }

    fn record(timestamp_micros: u64, source_file: &str, source_offset: usize) -> LogRecord {
        LogRecord {
            timestamp_micros,
            level: "INFO".to_owned(),
            name: "APP".to_owned(),
            tag: "test".to_owned(),
            thread_id: 1,
            is_main_thread: true,
            message: "message".to_owned(),
            source_file: source_file.to_owned(),
            source_offset,
        }
    }
}
