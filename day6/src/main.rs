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
                    let input_as_vec_chars: Vec<_> = input.chars().collect();
                    let vec_length:usize = input_as_vec_chars.len();
                    let mut i:usize = 0;
                    while i < vec_length {
                        print!("{} -",i );
                        if !has_dupes_new(input_as_vec_chars[i], input_as_vec_chars[i+1], input_as_vec_chars[i+2], input_as_vec_chars[i+3],input_as_vec_chars[i+4], input_as_vec_chars[i+5], input_as_vec_chars[i+6], input_as_vec_chars[i+7],input_as_vec_chars[i+8], input_as_vec_chars[i+9], input_as_vec_chars[i+10], input_as_vec_chars[i+11], input_as_vec_chars[i+12], input_as_vec_chars[i+13]){
                            println!("{}",i+14);
                            break;
                        }
                        // if !has_dupes(input_as_vec_chars[i], input_as_vec_chars[i+1], input_as_vec_chars[i+2], input_as_vec_chars[i+3]){
                        //     println!("i: {} - {}{}{}{}",i,input_as_vec_chars[i], input_as_vec_chars[i+1], input_as_vec_chars[i+2], input_as_vec_chars[i+3]);
                        //     break;
                        // }
                        i +=1;
                    }
                    println!("vec length {}",vec_length);
                }
            }
        }
    }
}
fn has_dupes_new(a:char, b:char, c:char, d:char, e:char, f:char, g:char, h:char, i:char, j:char, k:char, l:char, m:char, n:char)-> bool{
    println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",a,b,c,d,e,f,g,h,i,j,k,l,m,n);
    if a == b || a == c || a == d || a == e || a == f || a == g || a == h || a == i || a == j || a == k || a == l || a == m || a == n {
        println!("a");
        return true;
    }else if b == c || b == d || b == e || b == f || b == g || b == h || b == i || b == j || b == k || b == l || b == m || b == n {
        println!("b");
        return true;
    }else if c == d || c == e || c == f || c == g || c == h || c == i || c == j || c == k || c == l || c == m || c == n {
        println!("c");
        return true;
    }else if d == e || d == f || d == g || d == h || d == i || d == j || d == k || d == l || d == m || d == n {
        println!("d");
        return true;
    }else if e == f || e == g || e == h || e == i || e == j || e == k || e == l || e == m || e == n {
        println!("e");
        return true;
    }else if f == g || f == h || f == i || f == j || f == k || f == l || f == m || f == n {
        println!("f");
        return true;
    }else if g == h || g == i || g == j || g == k || g == l || g == m || g == n {
        println!("g");
        return true;
    }else if h == i || h == j || h == k || h == l || h == m || h == n {
        println!("h");
        return true;
    }else if i == j || i == k || i == l || i == m || i == n {
        println!("i");
        return true;
    }else if j == k || j == l || j == m || j == n {
        println!("j");
        return true;
    }else if k == l || k == m || k == n {
        println!("k");
        return true;
    }else if l == m || l == n {
        println!("l");
        return true;
    }else if m == n {
        println!("l");
        return true;
    }else {
        return false;
    }
}

fn has_dupes(a:char, b:char, c:char, d:char)-> bool{
    if a == b || a == c || a == d || b == c || b == d || c == d {
        return true;
    }else{
        return false;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>,{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}