mod crypto;
mod decoder;
mod logger;
mod log_serialize_generated;

use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use clap::{Arg, Command};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 检查是否没有提供任何参数
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        // 没有提供任何参数，显示完整帮助信息
        let mut cmd = Command::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .about("MxLogger Analyzer - A tool for decoding and analyzing encrypted log files")
            .long_about("MxLogger Analyzer is a Rust-based tool designed to decrypt and analyze log files.\n\
                         It supports AES decryption and can parse log data using FlatBuffers format.\n\
                         The tool outputs decoded logs with proper timestamp formatting and timezone support.\n\n\
                         ENVIRONMENT VARIABLES:\n  \
                         MXLOGGER_CRYPT_KEY - Encryption key for AES decryption\n  \
                         MXLOGGER_IV_KEY    - Initialization vector for AES decryption\n\n\
                         Both environment variables are required for decryption.")
            .after_help("EXAMPLES:\n  \
                         export MXLOGGER_CRYPT_KEY=DMCNytVfhKUqPzyY8Bad\n  \
                         export MXLOGGER_IV_KEY=f3cufTR8m8XNwZYo4LGM\n  \
                         mxlogger_analyzer_rust logfile.bin\n  \
                         mxlogger_analyzer_rust logfile.bin --timezone Asia/Shanghai\n  \
                         mxlogger_analyzer_rust logfile.bin -o custom_output --verbose\n  \
                         mxlogger_analyzer_rust logfile.bin -z UTC -o decoded_logs -v")
            .arg(Arg::new("input")
                .help("Path to the input log file to decode")
                .long_help("Specify the path to the encrypted log file that needs to be decoded.\n\
                           The file should be in the format supported by MxLogger.\n\
                           Example: /path/to/logfile.bin")
                .required(true)
                .index(1)
                .value_name("FILE"))
            .arg(Arg::new("timezone")
                .long("timezone")
                .short('z')
                .help("Specify the timezone for timestamp formatting")
                .long_help("Set the timezone for displaying timestamps in the decoded logs.\n\
                           Accepts standard timezone identifiers like 'Asia/Shanghai', 'UTC', 'America/New_York'.\n\
                           If not specified, the system default timezone will be used.")
                .value_name("TIMEZONE")
                .num_args(1))
            .arg(Arg::new("output")
                .long("output")
                .short('o')
                .help("Specify output file name (without extension)")
                .long_help("Set a custom name for the output file. If not specified, the input file name will be used.\n\
                           The output will be saved as a .txt file with decoded log content.")
                .value_name("NAME")
                .num_args(1))
            .arg(Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Enable verbose output")
                .long_help("Enable detailed logging output to show the decoding process,\n\
                           including progress information and debugging details.")
                .action(clap::ArgAction::SetTrue))
            .help_template("{before-help}{name} {version}\n{author-with-newline}{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}");
        
        cmd.print_help()?;
        println!(); // 添加一个换行
        std::process::exit(1);
    }

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("MxLogger Analyzer - A tool for decoding and analyzing encrypted log files")
        .long_about("MxLogger Analyzer is a Rust-based tool designed to decrypt and analyze log files.\n\
                     It supports AES decryption and can parse log data using FlatBuffers format.\n\
                     The tool outputs decoded logs with proper timestamp formatting and timezone support.\n\n\
                     ENVIRONMENT VARIABLES:\n  \
                     MXLOGGER_CRYPT_KEY - Encryption key for AES decryption\n  \
                     MXLOGGER_IV_KEY    - Initialization vector for AES decryption\n\n\
                     Both environment variables are required for decryption.")
        .after_help("EXAMPLES:\n  \
                     export MXLOGGER_CRYPT_KEY=DMCNytVfhKUqPzyY8Bad\n  \
                     export MXLOGGER_IV_KEY=f3cufTR8m8XNwZYo4LGM\n  \
                     mxlogger_analyzer_rust logfile.bin\n  \
                     mxlogger_analyzer_rust logfile.bin --timezone Asia/Shanghai\n  \
                     mxlogger_analyzer_rust logfile.bin -o custom_output --verbose\n  \
                     mxlogger_analyzer_rust logfile.bin -z UTC -o decoded_logs -v")
        .arg(Arg::new("input")
            .help("Path to the input log file to decode")
            .long_help("Specify the path to the encrypted log file that needs to be decoded.\n\
                       The file should be in the format supported by MxLogger.\n\
                       Example: /path/to/logfile.bin")
            .required(true)
            .index(1)
            .value_name("FILE"))
        .arg(Arg::new("timezone")
            .long("timezone")
            .short('z')
            .help("Specify the timezone for timestamp formatting")
            .long_help("Set the timezone for displaying timestamps in the decoded logs.\n\
                       Accepts standard timezone identifiers like 'Asia/Shanghai', 'UTC', 'America/New_York'.\n\
                       If not specified, the system default timezone will be used.")
            .value_name("TIMEZONE")
            .num_args(1))
        .arg(Arg::new("output")
            .long("output")
            .short('o')
            .help("Specify output file name (without extension)")
            .long_help("Set a custom name for the output file. If not specified, the input file name will be used.\n\
                       The output will be saved as a .txt file with decoded log content.")
            .value_name("NAME")
            .num_args(1))
        .arg(Arg::new("verbose")
            .long("verbose")
            .short('v')
            .help("Enable verbose output")
            .long_help("Enable detailed logging output to show the decoding process,\n\
                       including progress information and debugging details.")
            .action(clap::ArgAction::SetTrue))
        .help_template("{before-help}{name} {version}\n{author-with-newline}{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}")
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let timezone = matches.get_one::<String>("timezone").map(|s| s.as_str());
    let verbose = matches.get_flag("verbose");
    let custom_output = matches.get_one::<String>("output");
    let input_file = Path::new(input_path);
    
    if verbose {
        println!("=== MxLogger Analyzer Parameters ===");
        println!("Input file: {}", input_path);
        println!("Timezone: {}", timezone.unwrap_or("System default"));
        println!("Custom output: {}", custom_output.map(|s| s.as_str()).unwrap_or("Auto-generated from input filename"));
        println!("Verbose mode: Enabled");
        println!("=====================================");
        println!();
    }
    
    // 获取输出文件名
    let output_filename = if let Some(custom_name) = custom_output {
        custom_name.as_str()
    } else {
        input_file
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output")
    };
    
    if verbose {
        println!("Final output filename: {}.txt", output_filename);
        println!("Reading input file...");
    }

    let mut file = File::open(input_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    if verbose {
        println!("File size: {} bytes", buffer.len());
        println!("Starting decoding process...");
    }

    // 从环境变量获取密钥
    let crypt_key = std::env::var("MXLOGGER_CRYPT_KEY").ok();
    let iv_key = std::env::var("MXLOGGER_IV_KEY").ok();
    
    if verbose {
        println!("Crypt key from env: {}", 
                 if crypt_key.is_some() { "✅ Found" } else { "❌ Not found" });
        println!("IV key from env: {}", 
                 if iv_key.is_some() { "✅ Found" } else { "❌ Not found" });
    }

    let result = decoder::decode(
        &buffer,
        crypt_key.as_deref(),
        iv_key.as_deref(),
        output_filename,
        timezone, // Pass timezone argument
        |error_code, error_msg| {
            eprintln!("🚨 Decoding Error ({}): {}", error_code, error_msg);
            if verbose {
                match error_code {
                    1 => eprintln!("   💡 This usually indicates data corruption or incorrect file format"),
                    2 => eprintln!("   💡 This might be related to encryption key mismatch"),
                    _ => eprintln!("   💡 Unknown error type, check file integrity"),
                }
            }
        },
        || {
            eprintln!("⚠️  Repeated error occurred - continuing with next data block");
        },
        |total| {
            if verbose {
                println!("📊 Total logs discovered: {}", total);
            }
        },
        |current, total| {
            if verbose {
                let percentage = if total > 0 { (current * 100) / total } else { 0 };
                println!("⏳ Progress: {}/{} ({}%)", current, total, percentage);
            }
        },
    ).await;

    result?;

    if verbose {
        println!("✅ Decoding completed successfully!");
        println!("📁 Output saved to: {}.txt", output_filename);
    } else {
        println!("✅ Decoding completed. Output: {}.txt", output_filename);
    }

    Ok(())
}
