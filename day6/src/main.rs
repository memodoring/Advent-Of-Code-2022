use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args:Vec<String> = env::args().collect();
    let file_path = &args[1];
    if let Ok(lines) =  read_lines(file_path){
        for line in lines {
            if let Ok(input) = line {
                if !input.is_empty(){
                    println!("{}", input);
                    let input_as_vec_chars: Vec<&char> = input.chars().collect();
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}