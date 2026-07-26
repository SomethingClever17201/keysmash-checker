use std::collections::HashMap;
use std::error::Error;
use std::fs;


fn main() -> Result<(), Box<dyn Error>> {

    let message:  String = fs::read_to_string("out.csv")?;
    println!("{}",message);
    Ok(())
}

