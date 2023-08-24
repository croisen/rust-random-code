use std::{
    collections::HashMap,
    fs::File,
    io::{ BufRead, BufReader }
};
use once_cell::sync::Lazy;


pub static CORRECT_HASHES: Lazy<HashMap<i64, String>> = Lazy::new(|| {
    let mut hashes: HashMap<i64, String> = HashMap::new();
    let hashes_file = BufReader::new(
        File::open("./hashes/project_euler_answers.txt").unwrap())
        .lines();

    for (problem_num, hash) in hashes_file.enumerate() {
        let hash = hash.unwrap();
        hashes.insert((problem_num as i64) + 1 , hash);
    }

    hashes
});
