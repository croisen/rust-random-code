mod hashes;
mod prb_001_005;
use crate::{
    func_exec::exec,
    project_euler_solutions::extra_funcs::{ art_thou_prime }
};


#[test]
fn test_unsolved() {
    let unsolved: String = exec(i32::MAX, false);
    assert_eq!(unsolved, "I hab no solution".to_string());
}

#[test]
fn test_prime_checker() {
    assert!   (art_thou_prime( 1).is_err());
    assert_eq!(art_thou_prime(69).unwrap(), false);
    assert_eq!(art_thou_prime(23).unwrap(),  true);
}
