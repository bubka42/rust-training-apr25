use p22::calc;

#[test]
fn test_celsius2farenheit() {
    // Test the conversion from Celsius to Farenheit
    let celsius = 35;
    let farenheit = calc::celsius2farenheit(celsius);
    assert_eq!(farenheit, 95);
}

#[test]
fn test_farenheit2celsius() {
    // Test the conversion from Farenheit to Celsius
    let farenheit = 95;
    let celsius = calc::farenheit2celsius(farenheit);
    assert_eq!(celsius, 35);
}

#[test]
fn test_fibonacci_loop() {
    // Test the Fibonacci sequence using loop
    let n = 15;
    let fib = calc::fibonacci_loop(n);
    assert_eq!(fib, 610);
}

#[test]
fn test_fibonacci_rec() {
    // Test the Fibonacci sequence using recursion
    let n = 15;
    let fib = calc::fibonacci_rec(n);
    assert_eq!(fib, 610);
}
