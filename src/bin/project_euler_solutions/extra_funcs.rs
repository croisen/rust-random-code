pub fn art_thou_prime(num: i64) -> Result<bool, String> {
    if num == 1 {
        return Err("The number 1 is neither a prime nor a composite number".to_string());
    } else if (num % 2 == 0) && (num != 2) {
        return Ok(false);
    }
    let mut res = (3..((f64::sqrt(num as f64)) as i64) + 1)
        .step_by(2)
        .filter(|&x| (num % x == 0) && (num != x))
        .peekable();
    Ok(res.peek().is_none())
}

pub fn reverse_an_integer(num: i64) -> i64 {
    let mut res: i64 = 0;
    let mut new_num: i64 = num.clone();
    while new_num > 0 {
        res *= 10;
        res += new_num % 10;
        new_num /= 10;
    }
    res
}

pub fn is_divisible_by_1_through_n(num: i64, n: i64) -> bool {
    let mut res = (2..n + 1).filter(|x| num % x != 0).peekable();
    return res.peek().is_none();
}
