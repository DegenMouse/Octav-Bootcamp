//tutorial-pipeline-pop-01.rs

use serde::{Deserialize, Serialize};
use std::{env, error::Error, ffi::OsString, fs::File, process};


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RecordD {
    city: String,
    state: String,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RecordE<'a> {
    city: &'a str,
    state: &'a str,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

fn decode(path: String) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record:RecordD = result?;
        println!("{:?}",record)

    }
    Ok(())
}

fn encode(path: String) -> Result<(), Box<dyn Error>> {
    
    let file = File::create(path)?;
    let mut wtr = csv::Writer::from_writer(file);

    wtr.serialize(RecordE {
        city: "Davidsons Landing",
        state: "AK",
        population: None,
        latitude: 65.2419444,
        longitude: -165.2716667,
    })?;
    wtr.serialize(RecordE {
        city: "Kenai",
        state: "AK",
        population: Some(7610),
        latitude: 60.5544444,
        longitude: -151.2583333,
    })?;
    wtr.serialize(RecordE {
        city: "Oakman",
        state: "AL",
        population: None,
        latitude: 33.7133333,
        longitude: -87.3886111,
    })?;

    wtr.flush()?;
    Ok(())
}

fn get_args() -> Result<(OsString, OsString), Box<dyn Error>> {
    match (env::args_os().nth(1), env::args_os().nth(2)) {
        (Some(file_path), Some(command)) => Ok((file_path, command)),
        _ => Err(From::from("expected two arguments: <file_path> <command>")),
    }
}

fn main() {
    let args = match get_args() {
        Ok(args) => args,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let command = args.0.into_string().unwrap();

    if command == "decode" {
        if let Err(err) = decode(args.1.into_string().unwrap()) {
            println!("{}", err);
            process::exit(1);
        }
    }
    else if command == "encode" {
        if let Err(err) = encode(args.1.into_string().unwrap()) {
            println!("{}", err);
            process::exit(1);
        }
    }

    
}
