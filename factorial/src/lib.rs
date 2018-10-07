#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(39916800, recurive_factorial(11));
        assert_eq!(39916800, iterative_factorial(11));
        assert_eq!(39916800, iterator_factorial(11));
    }

    fn recurive_factorial(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            n * recurive_factorial(n - 1)
        }
    }

    fn iterative_factorial(n: u32) -> u32 {
        let mut i = 1u32;
        let mut result = 1u32;

        while i <= n {
            result *= i;
            i += 1;
        }

        result
    }

    fn iterator_factorial(n: u32) -> u32 {

        // create a slice and iter it then fold
        (1..n + 1).fold(1, |acc, x| {
            acc * x
        })
    }
}
