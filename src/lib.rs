pub mod calc {

    // Function to sum two numbers and add 1 to the result
    pub fn sum_plus_one(x: u8, y: u8) -> u8 {
        x + y + 1  // Simply adds x, y, and 1
    }

    // Function to subtract two numbers and subtract 1 more, with a floor at 0
    pub fn sub_less_one(x: u8, y: u8) -> u8 {
        if x <= y {  // If x is less than or equal to y
            0       // Return 0 to prevent underflow
        } else {
            x - y - 1  // Otherwise, subtract y and 1 from x
        }
    }

    // The test module, only compiled and run when testing
    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_sum() {
            let result = sum_plus_one(5, 6);  // Call the function with 5 and 6
            let expected = 12;  // Expect the result to be 12
            assert_eq!(result, expected);  // Check if result matches expectation
        }

        #[test]
        fn test_sub() {
            let result = sub_less_one(10, 6);  // Call the function with 10 and 6
            let expected = 3;  // Expect the result to be 3 (10 - 6 - 1)
            assert_eq!(result, expected);  // Check if result matches expectation

            let result = sub_less_one(5, 5);  // Call the function with 5 and 5
            let expected = 0;  // Expect the result to be 0, since x equals y
            assert_eq!(result, expected);  // Check if result matches expectation
        }

        // Test case for sub_less_one function where the result should fail
        #[test]
        fn test_sub_failed() {
            let result = sub_less_one(4, 6);  // Call the function with 4 and 6
            let expected = 0;  // Expect the result to be 0, since x is less than y
            assert_eq!(result, expected);  // Check if result matches expectation

            let result = sub_less_one(0, 1);  // Call the function with 0 and 1
            let expected = 0;  // Expect the result to be 0, since x is less than y
            assert_eq!(result, expected);  // Check if result matches expectation
        }
    }
}
