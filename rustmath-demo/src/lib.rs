#[unsafe(no_mangle)]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[unsafe(no_mangle)]
pub extern "C" fn greet(ret_ptr: *mut u8, sub: &str) {
    let response = format!("Hello, {}", sub);

    unsafe {
        std::ptr::copy_nonoverlapping(response.as_ptr(), ret_ptr, response.len());
        *ret_ptr.add(response.len()) = 0; // null terminator
    }
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
