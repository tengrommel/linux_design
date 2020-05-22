use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

enum Section {
    Header,
    Data,
    Extension,
}

struct Record {
    key: String,
    value: String,
}

const BLOCK_SIZE: usize = 2880;
const RECORD_SIZE: usize = 80;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file_name = if args.len() > 1 {
        args[1].to_string()
    } else {
        "data/HRSz0yd020fm_c2f.fits".to_string()
    };
    let data_path = Path::new(&file_name);
    let mut file = File::open(data_path).expect("Couldn't open data fits");
    let mut fits_data = Vec::new();
    let bytes = file
        .read_to_end(&mut fits_data)
        .expect("Couldn't read data file");
    println!(
        "Successful read {} bytes from {}",
        bytes,
        data_path.display()
    );
    let mut state = Section::Header;
    let mut block_index = 0;

    let _chunks: Vec<()> = fits_data
        .chunks(BLOCK_SIZE)
        .enumerate()
        .map(|(i, chunk)| {
            println!("Processing chunk, {}", i);
            match state {
                Section::Header => {
                    if String::from_utf8_lossy(chunk).contains(" END ") {
                        parse_header(&fits_data[block_index..(block_index + (i + 1) * BLOCK_SIZE)]);
                        state = Section::Data;
                        block_index = i;
                    }
                }
                Section::Data => {
                    let data_size =
                        parse_data(&fits_data[block_index..(block_index + (i + 1) * BLOCK_SIZE)]);
                    state = Section::Extension;
                    block_index = i;
                }
                Section::Extension => {
                    if String::from_utf8_lossy(chunk).contains("XTENSION") {
                        println!("[Found extension start]");
                        state = Section::Extension;
                    }
                    // @todo it seems possible to hit this block and try and parse
                    if String::from_utf8_lossy(chunk).contains(" END ") {
                        parse_extension(
                            &fits_data[block_index..(block_index + (i + 1) * BLOCK_SIZE)],
                        );
                        state = Section::Header;
                        block_index = i;
                    }
                }
            };
        })
        .collect();
}

fn parse_data(data: &[u8]) -> usize {
    0
}

fn parse_extension(data: &[u8]) {
    println!("[Parsing data]");
}

fn parse_record(record: Cow<str>) -> Option<Record> {
    if record.contains('=') {
        let records: Vec<&str> = record.splitn(2, "=").collect();
        let r = Record {
            key: records[0].trim().to_string(),
            value: records[1].trim().to_string(),
        };
        Some(r)
    } else {
        None
    }
}

fn parse_header(data: &[u8]) {
    let mut header_records = HashMap::new();
    println!("Found header end");
    for (i, _) in data.iter().enumerate().step_by(RECORD_SIZE) {
        let record = &data[i..(i + RECORD_SIZE)];
        let record_string = String::from_utf8_lossy(record);
        if let Some(Record { key, value }) = parse_record(record_string) {
            // println!("{}: {}", key, value);
            header_records.insert(key, value);
        }
    }
}

// fn stringify(data: &[u8]) -> String {
// String::from_utf8_lossy(data).into_owned()
// }
