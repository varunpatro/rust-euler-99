pub fn p01() -> u64 {
    let mut sum  = 0;
    for i in 1..1000 {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        }
    }

    return sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p01() {
        assert_eq!(p01(), 233168);
    }
}
