#[cfg(test)]

mod tests {
    use crate::config::Config;

    fn run_test(expression: &str, expected: f64) {
        assert_eq!(
            calculate(expression, Identifiers::get(), Config::default()),
            expected
        );
    }

    #[test]
    fn my_test() {
        run_test("3 + 4", 7.0);
    }
}
