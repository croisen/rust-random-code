use rug::Integer;
use crate::project_euler_solutions::problem_001_005::{ problem_1, problem_2 };


pub fn exec(problem: Integer, verbose: bool) {
    match problem.to_i128_wrapping() {
        // I dunno how to make a function array and a macro to call
        // the function from said array :(
        1 => drop(problem_1(verbose)),
        2 => drop(problem_2(verbose)),
        _ => println!("Sadly croisen have not solved problem #{} yet...", problem),
    }
}
