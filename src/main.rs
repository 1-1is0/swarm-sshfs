use std::fs::File;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("Search file {} for {}", filename, query);
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.eq(query) {
                    println!("{} is equal to query {}", ip, ip.eq(query));
                    // println!
                }
                
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


// use std::env;
// use std::fs;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let query = &args[1];
//     let filename = &args[2];

//     println!("Searching for {}", query);
//     println!("In file {}", filename);


//     let contents = fs::read_to_string(filename)
//         .expect("Something went wrong reading the file");

//     println!("With text:\n{}", contents);
// }