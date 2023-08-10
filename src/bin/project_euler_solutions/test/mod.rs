mod hashes;
mod prb_001_005;
mod prb_006_010;

use crate::{
    func_exec::exec,
    project_euler_solutions::extra_funcs::{ art_thou_prime, reverse_an_integer }
};


#[test]
fn test_unsolved() {
    let unsolved: String = exec(i32::MAX, false);
    assert_eq!(unsolved, "I hab no solution".to_string());
}

#[test]
fn test_prime_checker() {
    assert!   (art_thou_prime( 1).is_err());
    assert_eq!(art_thou_prime(15).unwrap(), false);
    assert_eq!(art_thou_prime(23).unwrap(),  true);
}

#[test]
fn test_reverse_a_number() {
    assert_eq!(reverse_an_integer(1234), 4321);
}
