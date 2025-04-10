mod math;
mod messages;

fn main() {
    let result = math::add(3, 4);
    println!("Sum: {}", result);

    messages::greetings::hello("Rustacean");
}
``