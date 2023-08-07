pub fn art_thou_prime(num: i64) -> Result<bool, String> {
    if num == 1 {
        return Err("The number 1 is neither a prime nor a composite number".to_string());
    } else if num % 2 == 0 {
        return Ok(false);
    }

    let mut res = (2..(f64::sqrt(num as f64)) as i64)
        .filter(|x| x % 2 != 0)
        .filter(|x| num % x == 0)
        .peekable();

    return Ok(res.peek().is_none());
}
