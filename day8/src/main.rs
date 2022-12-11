use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args:Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut trees = HashMap::new();
    if let Ok(lines) =  read_lines(file_path){
        let mut for_counter :u16 = 0;
        for line in lines {
            if let Ok(input) = line {
                if !input.is_empty(){
                    let input_as_vec_chars: Vec<char> = input.chars().collect();
                    trees.insert(for_counter, &input_as_vec_chars);
                    println!("{:?}",for_counter);
                    //println!("{:?}",left_find_largest(&input_as_vec_chars));
                    // left_find_largest(input_as_vec_chars);
                    for_counter += 1;
                }
                trees.iter().map(|s|{println!("{:?}",left_find_largest(s.1));});
            }
        }
    }
}

fn left_find_largest(vec: &Vec<char>) -> (usize, usize){
    let mut i: usize = 0;
    let mut b:usize = 0;
    let mut max_value:usize = vec[0].to_digit(10).unwrap() as usize;
    let mut max_value_position:usize = 0;
    while i < vec.len()-1{
        b = vec[i].to_digit(10).unwrap() as usize; //potential length issue if the largest is the right edge
        if b > max_value{
            max_value = b;
            max_value_position = i;
        }
        i +=1;
    }
    return (max_value, max_value_position);
}

// fn right_find_largest(vec: &Vec<char>) -> (usize, usize){
//     let row_length :usize = vec.len();
//     let (value, position) = left_find_largest(vec.into_iter().rev().collect());
//     let row_length_i32:i32 = row_length.try_into().unwrap();
//     let position_i32:i32 = position.try_into().unwrap();
//     let adjusted_position_i32: i32 = position_i32 - row_length_i32 +1;
//     let abs_adjusted_position: usize = adjusted_position_i32.abs() as usize;
//     return (value, abs_adjusted_position);
// }

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}