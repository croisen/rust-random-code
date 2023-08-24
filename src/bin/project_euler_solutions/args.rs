use std::process::exit;

use argparse::{ ArgumentParser, StoreTrue, Store };


pub struct Args {
    pub verbose: bool,
    pub problem: i64,
}

pub fn parse_args() -> Args {
    let mut args = Args {
        verbose: false,
        problem: 1,
    };

    {
        let mut parser: ArgumentParser = ArgumentParser::new();
        parser.set_description("Croisen's solution in some of the problems in projecteuler.net");
        parser.refer(&mut args.verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
                "Prints a more detailed output of what is currently happening with the program.");
        parser.refer(&mut args.problem)
            .add_option(&["-p", "--problem-number"], Store,
                "Sets which problem is to be solved and searches if there's a function that solves it");
        parser.parse_args_or_exit();

    }

    return args;
}
