use crate::project_euler_solutions::{
    problem_001_005::{ problem_1, problem_2, problem_3, problem_4, problem_5 }
};


pub fn exec(problem: i32, verbose: bool) -> String {
    match problem {
        // I dunno how to make a function array and a macro to call
        // the function from said array :(
        1 => problem_1(verbose),
        2 => problem_2(verbose),
        3 => problem_3(verbose),
        4 => problem_4(verbose),
        5 => problem_5(verbose),
        _ => { 
            println!("Sadly croisen have not solved problem #{} yet...", problem);
            "I hab no solution".to_string()
        },
    }
}
