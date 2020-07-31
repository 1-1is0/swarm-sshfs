// use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
// use std::io::{self, BufRead};
use std::path::Path;
use clap::{App, load_yaml};

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let function = &args[1];
    // let filename = &args[2];
    // let query = &args[3];
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    // let Some(ops) = matches.value_of("options");
    // println!("ops {}", matches.value_of("option").unwrap());
    let func = matches.value_of("code");
    let filename = matches.value_of("filename");
    let query = matches.value_of("query");

    println!("func {} filename {} query {}", func.unwrap(), filename.unwrap(), query.unwrap());

    if let Some(func) = matches.value_of("code") {
        match func {
            "search" => search(filename.unwrap().to_string(), query.unwrap().to_string()),
            "generate" => generate_file(filename.unwrap().to_string(), query.unwrap().to_string().parse::<i32>().unwrap()),
            _ => unreachable!(),
        }
        
    // } else if function.eq("generate") {
        
    }

}

fn search(filename:String, query: String)  {
    // File hosts must exist in current path before this produces output
    // query = "number " + query;
    let search_query = format!("write line {} number {}", query, query);
    let filepath = Path::new("/home/data/").join(filename);
    println!("Search file {} for '{}'", filepath.display(), search_query);
    if let Ok(lines) = read_lines(filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.eq(&search_query) {
                    println!("Found {} in file", query);
                    // println!
                    break;
                }
                
            }
        }
    }
    // println!("Not found {} in file", query)
    // return 1;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn generate_file(filename: String, end_number: i32) {
    // Create a temporary file.
    let d  = Path::new("/home/data/").join(filename);
    // let temp_file = temp_directory.join("data").join(filename);
    // println!("file {}", temp_file.to_str().unwrap());
    println!("file {}", d.to_str().unwrap());

    // // Open a file in write-only (ignoring errors).
    // // This creates the file if it does not exist (and empty the file if it exists).
    let mut file = File::create(d).unwrap();

    // // Write a &str in the file (ignoring the result).
    for i in 0..end_number {
        writeln!(&mut file, "write line {} number {}", i, i).expect("Could not write");
    }
}