mod project_euler_solutions;

use project_euler_solutions::{ args, func_exec };

fn main() {
    let args: args::Args = args::parse_args();
    println!("Trying to get the solution for https://projecteuler.net/problem={}",
        args.problem);
    func_exec::exec(args.problem, args.verbose);
}
