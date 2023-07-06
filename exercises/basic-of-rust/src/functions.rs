// Exercise 1
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// Exercise 2
pub fn sum_one_to_n(n: u32) -> u32 {
    (1..=n).sum()
}

// Exercise 3
fn calculate_average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    sum / count
}

// Exercise 4
fn sum_even_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().filter(|&num| num % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_should_work() {
        let (x, y) = (1, 2);
        let s = sum(x, y);
        assert_eq!(s, 3);
    }

    #[test]
    fn test_sum_0() {
        let result = sum_one_to_n(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_1() {
        let result = sum_one_to_n(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_sum_100() {
        let result = sum_one_to_n(100);
        assert_eq!(result, 5050);
    }

    #[test]
    fn test_calculate_average() {
        let numbers = [2.5, 4.8, 6.3, 1.7, 3.9];
        let result = calculate_average(&numbers);
        assert_eq!(result, 3.84);
    }

    #[test]
    fn test_calculate_average_empty() {
        let numbers: &[f64] = &[];
        let result = calculate_average(numbers);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sum_even_numbers() {
        assert_eq!(sum_even_numbers(&[1, 2, 3, 4, 5, 6]), 12);
        assert_eq!(sum_even_numbers(&[10, 20, 30, 40, 50]), 150);
        assert_eq!(sum_even_numbers(&[15, 25, 35, 45, 55]), 0);
        assert_eq!(sum_even_numbers(&[-2, 0, 2, 4, 6]), 10);
    }
}
