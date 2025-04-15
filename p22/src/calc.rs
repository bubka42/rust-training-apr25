// This is a simple calculator program in Rust that performs various calculations.

/// Celsius to Fahrenheit.
/// ```
/// let celsius = 25;
/// let farenheit = p22::calc::celsius2farenheit(celsius);
/// assert_eq!(farenheit, 77);
/// ```
pub fn celsius2farenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

/// Fahrenheit to Celsius.
/// ```
/// let farenheit = 77;
/// let celsius = p22::calc::farenheit2celsius(farenheit);
/// assert_eq!(celsius, 25);
/// ```
pub fn farenheit2celsius(farenheit: i32) -> i32 {
    (farenheit - 32) * 5 / 9
}

/// Fibonacci sequence using loop.
/// ```
/// let n = 10;
/// let fib = p22::calc::fibonacci_loop(n);
/// assert_eq!(fib, 55);
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        b += a;
        a = b - a;
    }
    a
}

/// Fibonacci sequence using recursion.
/// ```
/// let n = 10;
/// let fib = p22::calc::fibonacci_rec(n);
/// assert_eq!(fib, 55);
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 1) + fibonacci_rec(n - 2),
    }
}
