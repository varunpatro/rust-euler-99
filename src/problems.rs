pub fn p01() -> u64 {
    let mut sum  = 0;
    for i in 1..1000 {
        if (i % 3 == 0) || (i % 5 == 0) {
            sum += i;
        }
    }

    return sum
}

pub fn p02() -> u64 { return 0 }

pub fn p03() -> u64 { return 0 }

pub fn p04() -> u64 { return 0 }

pub fn p05() -> u64 { return 0 }

pub fn p06() -> u64 { return 0 }

pub fn p07() -> u64 { return 0 }

pub fn p08() -> u64 { return 0 }

pub fn p09() -> u64 { return 0 }

pub fn p10() -> u64 { return 0 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p01() {
        assert_eq!(p01(), 233168);
    }

    #[test]
    fn test_p02() {
        assert_eq!(p02(), 4613732);
    }

    #[test]
    fn test_p03() {
        assert_eq!(p03(), 6857);
    }

    #[test]
    fn test_p04() {
        assert_eq!(p04(), 906609);
    }

    #[test]
    fn test_p05() {
        assert_eq!(p05(), 232792560);
    }

    #[test]
    fn test_p06() {
        assert_eq!(p06(), 25164150);
    }

    #[test]
    fn test_p07() {
        assert_eq!(p07(), 104743);
    }

    #[test]
    fn test_p08() {
        assert_eq!(p08(), 23514624000);
    }

    #[test]
    fn test_p09() {
        assert_eq!(p09(), 31875000);
    }

    #[test]
    fn test_p10() {
        assert_eq!(p10(), 142913828922);
    }
}
