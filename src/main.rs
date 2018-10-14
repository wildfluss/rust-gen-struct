fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} filename struct", args[0]);
        std::process::exit(1);
    }

    println!("Hello, {}!", args[1]);
}
