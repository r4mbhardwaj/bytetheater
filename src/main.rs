fn store(filename: String) {
    println!("storing: {}", filename);
}

fn load(filename: String) {
    println!("loading: {}", filename);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("args: {:?}", args);

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let mode = args[1].clone();
    let filename = args[2].clone();

    match mode.as_str() {
        "store" => {
            if args.len() < 3 {
                println!("Please Please provide a file to store");
                return;
            }
            println!("store mode");
            store(filename);
        }
        "load" => {
            if args.len() < 3 {
                println!("Please provide the stored file to load");
                return;
            }
            println!("load mode");
            load(filename);
        }
        _ => {
            println!("unknown mode; must be 'store' or 'load'");
            std::process::exit(1);
        }
    }
}
