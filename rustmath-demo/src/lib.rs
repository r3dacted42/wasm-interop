#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[unsafe(no_mangle)]
pub extern "C" fn greet(sub: &str) -> String {
    format!("Hello, {}", sub)
}

#[unsafe(no_mangle)]
pub extern "C" fn is_even(num: i32) -> i32 {
    (num % 2 == 0) as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        f64::NAN
    } else {
        a / b
    }
}
