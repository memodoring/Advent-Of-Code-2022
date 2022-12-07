use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args:Vec<String> = env::args().collect();
    let file_path = &args[1];

    let vec_one = vec!['F','C','J','P','H','T','W','H'];
    let vec_two = vec!['G','R','V','F','Z','J','B'];
    let vec_three = vec!['H','P','T','R'];
    let vec_four = vec!['Z','S','N','P','H','T','C','D'];
    let vec_five = vec!['N','V','F','Z','H','J','Z'];
    let vec_six = vec!['P','M','G','F','W','D','D','P'];
    let vec_seven = vec!['M','V','Z','W','S','J'];
    let vec_eight = vec!['N','D','S'];
    let vec_nine = vec!['D','Z','S','F','M'];

    let (vec_one,vec_two) = move_from_to(vec_one, vec_two);
    println!("{:#?}",vec_one);
    println!("{:#?}",vec_two);
}
fn move_from_to(mut vec_from:Vec<char>, mut vec_to:Vec<char>)->(Vec<char>, Vec<char>){
    let value_to_move = vec_from.pop().unwrap();
    vec_to.push(value_to_move);
    return (vec_from, vec_to);
}
// [H]         [D]     [P]        
// [W] [B]     [C] [Z] [D]        
// [T] [J]     [T] [J] [D] [J]        
// [H] [Z]     [H] [H] [W] [S]     [M]
// [P] [F] [R] [P] [Z] [F] [W]     [F]
// [J] [V] [T] [N] [F] [G] [Z] [S] [S]
// [C] [R] [P] [S] [V] [M] [V] [D] [Z]
// [F] [G] [H] [Z] [N] [P] [M] [N] [D]
//  1   2   3   4   5   6   7   8   9 