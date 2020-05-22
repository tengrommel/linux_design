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

struct Fits<Header, Data, Extension> {
    header: Header,
    data: Data,
    extension: Vec<Extension>,
}

#[derive(Default)]
struct Header {
    data: HashMap<String, String>,
}

#[derive(Default)]
struct Data<'t> {
    data: &'t [u8],
}

struct Extension {
    data: String,
}

struct Record {
    key: String,
    value: String,
}

const BLOCK_SIZE: usize = 2880;
const RECORD_SIZE: usize = 80;

fn main() {
    let args: Vec<String> = env::args().collect();
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

    let mut fits: Fits<Header, Data, Extension> = Fits {
        header: Default::default(),
        data: Default::default(),
        extension: Default::default(),
    };

    for (current_block, chunk) in fits_data.chunks(BLOCK_SIZE).enumerate() {
        println!("Processing chunk, {}", current_block);
        match state {
            Section::Header => {
                if String::from_utf8_lossy(chunk).contains(" END ") {
                    fits.header = parse_header(&fits_data, block_index, current_block);
                    state = Section::Data;
                    block_index = current_block;
                }
            }
            Section::Data => {
                let dimensionality = 2;
                if dimensionality == 2 {
                    let (x, y) = (200, 2);
                    fits.data = parse_data(&fits_data, block_index, (x, y));
                }
                state = Section::Extension;
                block_index = current_block + (fits.data.data.len() / BLOCK_SIZE) + 1;
            }
            Section::Extension => {
                if String::from_utf8_lossy(chunk).contains("XTENSION") {
                    println!("[Found extension start]");
                    state = Section::Extension;
                }
                // @todo it seems possible to hit this block and try and parse
                if String::from_utf8_lossy(chunk).contains(" END ") {
                    fits.extension
                        .push(parse_extension(&fits_data, block_index, current_block));
                    state = Section::Header;
                    block_index = current_block;
                }
            }
        };
    }
}

fn parse_header(fits: &[u8], last_block: usize, current_block: usize) -> Header {
    let mut header_records = HashMap::new();
    let header_data =
        &fits[(last_block * BLOCK_SIZE)..(last_block + (current_block + 1) * BLOCK_SIZE)];
    println!("Found header end");
    for chunk in header_data.chunks(RECORD_SIZE) {
        let record_string = String::from_utf8_lossy(chunk);
        if let Some(Record { key, value }) = parse_record(record_string) {
            println!("{}: {}", key, value);
            header_records.insert(key, value);
        }
    }
    Header {
        data: header_records,
    }
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

fn parse_data(fits: &[u8], last_block: usize, (x, y): (u32, u32)) -> Data {
    let data_size = x * y * 4;
    let data_unit =
        &fits[(last_block * BLOCK_SIZE)..(data_size as usize + (last_block * BLOCK_SIZE))];
    Data { data: data_unit }
}

fn parse_extension(fits: &[u8], last_block: usize, current_block: usize) -> Extension {
    println!("[Parsing data]");
    let extension_data = &fits[(last_block * BLOCK_SIZE)..(current_block + 1) * BLOCK_SIZE];
    Extension {
        data: String::from_utf8_lossy(extension_data).to_string(),
    }
}
// fn stringify(data: &[u8]) -> String {
// String::from_utf8_lossy(data).into_owned()
// }
