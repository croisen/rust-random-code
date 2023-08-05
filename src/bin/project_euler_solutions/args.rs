use std::process::exit;

use argparse::{ ArgumentParser, StoreTrue, Store };
use rug::Integer;


pub struct Args {
    pub verbose: bool,
    pub problem: Integer,
}

pub fn parse_args() -> Args {
    let mut args = Args {
        verbose: false,
        problem: Integer::new(),
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

    if args.problem > i32::MAX {
        eprintln!("The argument {}, passed to --problem-number is not normal bud", args.problem);
        exit(1);
    }

    return args;
}
