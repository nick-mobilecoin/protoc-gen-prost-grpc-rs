use std::{env, io::{self, Read, Write, BufReader}, process::exit};

use anyhow::Result;

fn main() -> Result<()> {
    if env::args().any(|x| x == "--version") {
        println!(env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let stdin = BufReader::new(io::stdin().lock());
    let out = run_plugin(stdin)?;

    io::stdout().write_all(&out)?;
    Ok(())
}

fn run_plugin<R:Read>(_reader: BufReader<R>) -> Result<Vec<u8>> {
    Ok(b"what it is?".to_vec())
}