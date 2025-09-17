// We need to use the std::env::args function from the Standard Library;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // The first element will be the file path and my mini-grep will need
    // to
    let query = &args[1];
    let file_path = &args[2];

    print!("Query: {query}");
    print!("File path: {file_path}");
}
