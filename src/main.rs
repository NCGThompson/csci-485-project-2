use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file_path: String = args[1].to_owned();

    println!("{}", file_path);
}
