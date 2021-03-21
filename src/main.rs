use csv_parser::parse_line;
use payment_engine::ClientTable;
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};
mod client_info;
mod csv_parser;
mod currency;
mod payment_engine;
mod transaction;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please supply an csv file");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing csv file",
        ));
    }
    let mut client_table = ClientTable::new();

    let f = File::open(&args[1]).unwrap();
    let reader = BufReader::new(f);
    for tx in reader.lines().skip(1).map(parse_line) {
        if let Err(_e) = client_table.handle_transaction(tx?) {
            // From the task, we don't handle any of these errors
            // But in an actual setup we would probably log them or something
        }
    }

    println!("{}", client_table);
    Ok(())
}
