use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args.len() > 3 {
        println!("Usage: <src_filename> [dest_filename]");
        std::process::exit(1);
    }

    let src_filename = &args[1];
    let dest_filename = if args.len() > 2 {&args[2]} else {src_filename};
    
    println!("Converting {} to {}", src_filename, dest_filename);

    let src: String = fs::read_to_string(src_filename).expect("Unable to read source file");

    let dest_str = convert(src);

    let mut dest_file = File::create(dest_filename).expect("Failed to open destination file");
    write!(dest_file, "{}", dest_str).expect("Failed to write destination file");
}

fn convert(src: String) -> String {
    let mut out: String = Default::default();

    // TODO: processing creates destination string

    out
}