use std::fs::File;
use std::io::prelude::*;

fn read_file(path: String) -> std::vec::Vec<String> {
  let mut file = File::open(path).expect("file not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents)
      .expect("something went wrong reading the file");
  let contents_split = contents.split("\n");
  let vec_contents: Vec<String> = contents_split.map(|s| s.to_string()).collect(); 
  return vec_contents;
}

fn main() {
  let vec_nums_strs = read_file(String::from("input.txt"));
  let vec_nums: Vec<i32> = vec_nums_strs.iter().map(|s| if s.starts_with('-') { 
                                     let number_str: String = s.chars().skip(1).take(s.chars().count()).collect();
                                     let number: i32 = number_str.parse().unwrap();
                                     return number * -1;
                                 } else {
                                     let number_str: String = s.chars().skip(1).take(s.chars().count()).collect();
                                     let number: i32 = number_str.parse().unwrap();
                                     return number;
                                 }
                                 ).collect();
  let sum = vec_nums.iter().fold(0, |a, &b| a + b);
  println!("the sum is {}", sum);
}
