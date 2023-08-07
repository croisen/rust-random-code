use std::{
    collections::HashMap,
    fs::File,
    io::{ BufRead, BufReader }
};
use once_cell::sync::Lazy;


pub static CORRECT_HASHES: Lazy<HashMap<i32, String>> = Lazy::new(|| {
    let mut hashes: HashMap<i32, String> = HashMap::new();
    let hashes_file = BufReader::new(
        File::open("./hashes/project_euler_answers.txt").unwrap())
        .lines();

    for (problem_num, hash) in hashes_file.enumerate() {
        let hash = hash.unwrap();
        hashes.insert((problem_num as i32) + 1 , hash);
    }

    hashes
});
