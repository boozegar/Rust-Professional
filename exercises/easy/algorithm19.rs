/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number.
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.

    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};
// 定义矩阵乘法
fn matrix_mult(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    (
        a.0 * b.0 + a.1 * b.2,
        a.0 * b.1 + a.1 * b.3,
        a.2 * b.0 + a.3 * b.2,
        a.2 * b.1 + a.3 * b.3,
    )
}

// 定义矩阵快速幂
fn matrix_pow(matrix: (i32, i32, i32, i32), n: i32) -> (i32, i32, i32, i32) {
    if n == 1 || n == 0 {
        return matrix;
    }
    let half_pow = matrix_pow(matrix, n / 2);
    let result = matrix_mult(half_pow, half_pow);
    if n % 2 == 0 {
        result
    } else {
        matrix_mult(result, matrix)
    }
}

pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    if n == 0 {
        return 0;
    }
    let matrix = (1, 1, 1, 0);
    let result = matrix_pow(matrix, n - 1);
    result.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
