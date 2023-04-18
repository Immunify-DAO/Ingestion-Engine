use lib::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Not enough string");
    }

    print!("SHA-256 ({}) = ", args[1]);

    if let Ok(hash_arry) = hash_file(&args[1]) {
        print!("{}", hex_to_string(&hash_arry));
    }
}
