use sha256::digest;

use crate::project_euler_solutions::func_exec::exec;
use super::hashes::CORRECT_HASHES;

#[test]
fn test_prb6() {
    let hash: String = digest(exec(6, false));
    assert_eq!(hash, *CORRECT_HASHES.get(&6).unwrap());
}

#[test]
fn test_prb7() {
    let hash: String = digest(exec(7, false));
    assert_eq!(hash, *CORRECT_HASHES.get(&7).unwrap());
}

#[test]
fn test_prb8() {
    let hash: String = digest(exec(8, false));
    assert_eq!(hash, *CORRECT_HASHES.get(&8).unwrap());
}

#[test]
fn test_prb9() {
    let hash: String = digest(exec(9, false));
    assert_eq!(hash, *CORRECT_HASHES.get(&9).unwrap());
}

#[test]
fn test_prb10() {
    let hash: String = digest(exec(10, false));
    assert_eq!(hash, *CORRECT_HASHES.get(&10).unwrap());
}
