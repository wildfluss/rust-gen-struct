use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} filename struct", args[0]);
        std::process::exit(1);
    }

    println!("Hello, {}!", args[1]);

    let mut f = File::open(&args[1])?;
    let mut byte_vec = vec![];
    f.read_to_end(&mut byte_vec)?;

    println!("{:?}", byte_vec);
    Ok(())
}
