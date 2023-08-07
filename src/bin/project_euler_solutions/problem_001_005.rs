use crate::project_euler_solutions::extra_funcs;

use super::extra_funcs::art_thou_prime;

pub fn problem_1(verbose: bool) -> String {
    let mut result: i32 = 0;

    println!("If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6, and 9. The sum of these multiples is 23.");
    println!("Now find the sum of all the multiples of 3 or 5 below 1000");

    if verbose {
        (1..1000)
            .filter( |&x| ((x % 3 == 0) || (x % 5 == 0)) )
            .for_each( |x| {
                    println!("Current number: {:>3}, Current sum: {:>7}", x, result);
                    result += x;
            });
    } else {
        result = (1..1000)
            .filter( |&x| ((x % 3 == 0) || (x % 5 == 0)) )
            .sum();
    }

    println!("The sum of all numbers below 1000 that are divisible by 3 or 5 is {}", result);
    return result.to_string();
}

pub fn problem_2(verbose: bool) -> String {
    let mut a: i32      = 1;
    let mut b: i32      = 2;
    let mut result: i32 = 0;
    let mut c: i32;

    println!("Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first terms will be: 1, 2, 3, 5, 8, 13, 21, 34, 55, 89,...");
    println!("By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.");

    while a < 4000000 {
        if verbose { println!("Current fibonacci number: {:>7}", a); }
        c = a + b;
        a = b;
        b = c;

        if ( a % 2 == 0 ) && (a < 4000000) { result += a; }
    }

    println!("The sum of all even fibonacci numbers below 4 million is: {}", result);
    return result.to_string();
}

pub fn problem_3(verbose: bool) -> String {
    let big_num: i64 = 600_851_475_143;
    let mut result: i64  = 0;
    (3..(f64::sqrt(big_num as f64)) as i64)
        .step_by(2)
        .filter(|x| big_num % x == 0)
        .for_each( |x| {
            if art_thou_prime(x).unwrap() {
                result = x;
                if verbose {
                    println!("Current highest prime factor of {:>12} is {:>4}", big_num, result);
                }
            }
        });
    println!("The higest prime factor of {} is {}", big_num, result);
    return result.to_string();
}
