// Exercise 1
pub fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

// Exercise 2
fn check_number(number: i32) -> &'static str {
    if number > 0 {
        "Positive"
    } else if number < 0 {
        "Negative"
    } else {
        "Zero"
    }
}

// Exercise 3
pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    } else {
        "baz"
    }
}

// Exercise 4
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

// Exercise 5
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Exercise 6
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..(n / 2 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn test_check_number_positive() {
        let result = check_number(10);
        assert_eq!(result, "Positive");
    }

    #[test]
    fn test_check_number_negative() {
        let result = check_number(-5);
        assert_eq!(result, "Negative");
    }

    #[test]
    fn test_check_number_zero() {
        let result = check_number(0);
        assert_eq!(result, "Zero");
    }

    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }

    #[test]
    fn test_leap_year() {
        assert_eq!(is_leap_year(2020), true);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(1600), true);
    }

    #[test]
    fn test_non_leap_year() {
        assert_eq!(is_leap_year(2021), false);
        assert_eq!(is_leap_year(1900), false);
        assert_eq!(is_leap_year(1800), false);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_prime_number() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(19), true);
    }

    #[test]
    fn test_non_prime_number() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(10), false);
        assert_eq!(is_prime(15), false);
    }
}
