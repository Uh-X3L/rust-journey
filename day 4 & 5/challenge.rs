enum Command {
    Add(i32,i32),
    Sub(i32,i32),
    Exit
}
fn run_command(cmd:Command){
    match cmd {
        Command::Add(a,b) => println!("{} + {} = {}", a, b, a+b),
        Command::Sub(a,b) => println!("{} - {} = {}", a, b, a-b),
        Command::Exit => println!("Exiting..."),
    }
}
fn main() {
    let commands = vec![
        Command::Add(1, 2),
        Command::Sub(5, 3),
        Command::Exit,
    ]
    for cmd in commands {
        run_command(cmd);
    }