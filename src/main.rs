#[macro_use] extern crate lazy_static;
extern crate csv;
extern crate rustc_serialize;

mod record;

use std::collections::HashMap;
use record::Record;

fn main() {
    let path = std::env::args().nth(1).expect("provide filename");
    let records = pull_records(&path);
    
    match records {
        Err(e) => println!("error reading input: {}", e),
        Ok(records) => match group_records(records) {
            Err(e) => println!("failed to group records: {}", e),
            _ => (),
        }
    }
}

fn group_records<T>(records: T) -> Result<(), csv::Error>
    where T: IntoIterator<Item = Record>
{
    let grouped_records = records.into_iter()
        .fold(HashMap::new(), |mut map, item| {
            map.entry(item.key()).or_insert_with(|| Vec::new()).push(item);
            map
        });
        
    for (key, records) in grouped_records.iter() {
        let path = format!("apology.{}.csv", key);
        let mut writer = try!(csv::Writer::from_file(&path));
        
        for record in records {
            try!(writer.encode(record));
        }
    }
    
    Ok(())
}

fn pull_records(path: &str) -> Result<Vec<Record>, String> {
    let mut reader = csv::Reader::from_file(&path).unwrap();
    let mut results = vec![];
    
    let mut line = 1;
    for record in reader.decode() {
        line += 1;
        match record {
            Err(e) => return Err(format!("{}: {}", line, e)),
            Ok(record) => results.push(record),
        }
    }
    
    Ok(results)
}