use sha256::digest;

use crate::project_euler_solutions::{
    func_exec::exec,
    test::hashes::CORRECT_HASHES
};


#[test]
fn test_prb1() {
    let hash: String = digest(exec(1, false));
    assert_eq!(hash, *CORRECT_HASHES.get(&1).unwrap());
}

#[test]
fn test_prb2() {
    let hash: String = digest(exec(2, false));
    assert_eq!(hash, *CORRECT_HASHES.get(&2).unwrap());
}

#[test]
fn test_prb3() {
    let hash: String = digest(exec(3, false));
    assert_eq!(hash, *CORRECT_HASHES.get(&3).unwrap());
}
