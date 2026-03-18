use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

impl FileSize {
    fn get_bytes(&self) -> u64 {
        match self {
            FileSize::Bytes(bytes) => *bytes,
            FileSize::Kilobytes(kb) => (*kb * 1000.0) as u64,
            FileSize::Megabytes(mb) => (*mb * 1_000_000.0) as u64,
            FileSize::Gigabytes(gb) => (*gb * 1_000_000_000.0) as u64,
            FileSize::Terabytes(tb) => (*tb * 1_000_000_000_000.0) as u64,
        }
    }

    fn to_string_format(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
            FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
            FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
            FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
        }
    }
}

fn get_filesize_bytes(text: String) -> FileSize {
    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.len() != 2 {
        panic!("Invalid input format. Expected format: '<size> <unit>'");
    }
    let parsed_size: f64 = parts[0].parse().expect("Invalid size value");
    let unit = parts[1].to_string().to_uppercase();

    let size = match unit.as_str() {
        "BYTES" => parsed_size,
        "KB" => parsed_size * 1000.0,
        "MB" => parsed_size * 1_000_000.0,
        "GB" => parsed_size * 1_000_000_000.0,
        "TB" => parsed_size * 1_000_000_000_000.0,
        _ => panic!("Invalid unit. Expected one of: bytes, KB, MB, GB, TB"),
    };

    FileSize::Bytes(size as u64)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide an argument.");
        return;
    }
    println!("User argument: {}", args[1]);

    let filesize_in_bytes = get_filesize_bytes(args[1].clone());
    let bytes = filesize_in_bytes.get_bytes();

    println!(
        "Sizes {{ bytes: {}, kilobytes: {}, megabytes: {}, gigabytes: {}, terabytes: {} }}",
        bytes,
        FileSize::Kilobytes(bytes as f64 / 1000.0).to_string_format(),
        FileSize::Megabytes(bytes as f64 / 1_000_000.0).to_string_format(),
        FileSize::Gigabytes(bytes as f64 / 1_000_000_000.0).to_string_format(),
        FileSize::Terabytes(bytes as f64 / 1_000_000_000_000.0).to_string_format(),
    );
}
