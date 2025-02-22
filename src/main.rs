use num_bigint::BigUint;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
    let num = 100;
    let fact = factorial(num);
    println!("Factorial is: {}", fact);
}

fn factorial(num: u128) -> BigUint {
    let mut fact = BigUint::from(1u8);
    let mut temp_num = BigUint::from(num);
    while temp_num > BigUint::from(0u8) {
        fact *= &temp_num;
        temp_num -= BigUint::from(1u8);
    }
    fact
}
