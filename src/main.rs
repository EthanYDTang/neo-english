mod lib;
use lib::oxford::OxfordClient;

use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut client = OxfordClient{
        client: reqwest::Client::new()
    };
    let mut inp = String::new();
    io::stdin().read_line(&mut inp);
    println!("{}", client.get_word(&inp)?);
    Ok(())
}
