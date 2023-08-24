use super::{
    problem_001_005::*,
    problem_006_010::*
};

pub fn exec(problem: i64, verbose: bool) -> String {
    match problem {
        // I dunno how to make a function array and a macro to call
        // the function from said array :(
        1  => problem_1(verbose),
        2  => problem_2(verbose),
        3  => problem_3(verbose),
        4  => problem_4(verbose),
        5  => problem_5(verbose),
        6  => problem_6(verbose),
        7  => problem_7(verbose),
        8  => problem_8(verbose),
        9  => problem_9(verbose),
        10 => problem_10(verbose),
        _  => { 
            println!("Sadly croisen have not solved problem #{} yet...", problem);
            "I hab no solution".to_string()
        },
    }
}
