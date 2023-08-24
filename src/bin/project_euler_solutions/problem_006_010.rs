use super::extra_funcs;


pub fn problem_6(verbose: bool) -> String {
    println!("The sum of the squares of the first ten natural numbers is,");
    println!("1^2 + 2^2 + ... + 10^2 = 385");
    println!("The square of the sum of the first ten natural numbers is,");
    println!("(1 + 2 + ... + 10)^2 = 55^2 = 3025");
    println!("Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.");
    println!("Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.");

    let sum_of_sqared: i64 = (1..101)
        .map(|x| i64::pow(x, 2))
        .sum();

    let squared_sum  : i64 = (1..101)
        .sum::<i64>()
        .pow(2);
    if verbose {
        println!("The sum of the squares of the first 100 natural numbers is {}", sum_of_sqared);
        println!("The square of the sum of the first 100 natural numbers is {}", squared_sum);
    }

    let result: i64 = squared_sum - sum_of_sqared;
    println!("The difference betweem the sum of the squares of the first one hundred natural numbers and the square of the sum is {}", result);
    return result.to_string();
}

pub fn problem_7(verbose: bool) -> String {
    println!("By listing the first six prime numbers:");
    println!("2, 3, 5, 7, 11, and 13.");
    println!("We can see that the 6th prime is 13.");
    println!("What is the 10001st prime number?");

    let primes = (2i64..)
        .filter(|&x| extra_funcs::art_thou_prime(x).unwrap())
        .take(10001);
    if verbose {
        for (n, prime) in primes.clone().enumerate() {
            if        (n + 1) % 10 == 1 { println!("{:>5}st prime: {:>6}", n+1, prime); }
            else if   (n + 1) % 10 == 2 { println!("{:>5}nd prime: {:>6}", n+1, prime); }
            else if   (n + 1) % 10 == 3 { println!("{:>5}rd prime: {:>6}", n+1, prime); }
            else                        { println!("{:>5}th prime: {:>6}", n+1, prime); }
        }
    }
    let result: String = primes.last().unwrap().to_string();
    println!("The 10001st prime is {}", result);
    return result;
}

pub fn problem_8(verbose: bool) -> String {
    println!("The four adjacent digits in the 1000-digit number that have the greatest product are 9x9x8x9 = 5832.");
    println!("Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?");

    let long_num: String = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450".to_string();

    println!("The 1000-digit number: {}", long_num);

    let num_of_adjacent_digits: i64 = 13;
    let mut higest_result: i64 = 0;

    for x in 0..(1000 - num_of_adjacent_digits) {
        let mut result: i64 = 1;
        for y in 0..num_of_adjacent_digits {
            result *= long_num.chars().nth((x + y) as usize).unwrap().to_digit(10).unwrap() as i64;
        }
        if result > higest_result {
            higest_result = result;
            if verbose {
                println!("Current highest product of 13 adjacent numbers: {:>11}", higest_result);
            }
        }
    }
    println!("The highest product made by 13 adjacent numbers in the 1000-digit long number is {}", higest_result);
    return higest_result.to_string();
}

pub fn problem_9(verbose: bool) -> String {
    println!("A Pythagorean triplet is a set of three natural numbers, a < b <c, for which,");
    println!("a^2 + b^2 = c^2");
    println!("For example: 3^2 + 4^2 = 9 + 16 = 25 = 5^2");
    println!("There exists only one Pythagorean triplet for which a + b + c = 1000");
    println!("Find the product abc");

    let result: i64;
    for a in  1..1000i64                              {
    for b in (1..1000i64).skip(a.try_into().unwrap()) {
    for c in (1..1000i64).skip(b.try_into().unwrap()) {
        if a.pow(2) + b.pow(2) == c.pow(2) {
            if verbose {
                println!("Current Pythagorean triplet: a: {:>4} b: {:>4} c: {:>4}", a, b, c);
                println!("a^2: {} b^2 {} c^2 {}", a.pow(2), b.pow(2), c.pow(2));
            }
            if a + b + c == 1000 {
                if verbose {
                    println!("The only Pythagorean triplet whose sum is 1000 is:");
                    println!("a: {} b: {}, c: {}", a, b, c);
                }
                result = a*b*c;
                println!("The product of the numbers of the only Pythagorean whose sum is 1000 is {}", result);
                return result.to_string();
            }
        }
    }
    }
    }
    "I have no answer".to_string()
}

pub fn problem_10(verbose: bool) -> String {
    println!("The sum of the primes below 10 is 2 + 3 + 5 + 7 =  17.");
    println!("Find the sum of all the primes below two million.");

    if verbose {
        (2i64..2000000i64)
            .filter(|&x| extra_funcs::art_thou_prime(x).unwrap())
            .for_each(|y| {
                println!("Current prime number: {:>7}", y);
            })
    }
    let res: String = (2i64..2000000i64)
        .filter(|&x| extra_funcs::art_thou_prime(x).unwrap())
        .sum::<i64>()
        .to_string();
    println!("The sum of all prime numbers below 2 million is {}", res);
    return res;
}
