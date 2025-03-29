fn main() {
    let temp_celsius = 28.0;
    let temp_f = c_to_f(temp_celsius);
    println!("{}°C = {}°F", temp_celsius, temp_f);
}

fn c_to_f(c: f64) -> f64 {
    (c * 1.8) + 32.0
}
fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}
fn c_to_k(c: f64) -> f64 {
    c + 273.15
}
fn k_to_c(k: f64) -> f64 {
    k - 273.15
}
fn f_to_k(f: f64) -> f64 {
    c_to_k(f_to_c(f))
}
fn k_to_f(k: f64) -> f64 {
    c_to_f(k_to_c(k))
}
