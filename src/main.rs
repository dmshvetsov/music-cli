use std::env;
use std::process::{Command, Output};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_arg = &args[1];
    let output = match cmd_arg.as_str() {
        "play" => handle_command(cmd_arg),
        "pause" => handle_command(cmd_arg),
        "next" => handle_command(&String::from("next track")),
        "previous" => handle_command(&String::from("previous track")),
        _ => panic!("unknown command"),
    };
    print!("{}", String::from_utf8_lossy(&output.stdout));
    print!("{}", String::from_utf8_lossy(&output.stderr));
}

fn handle_command(cmd: &String) -> Output {
    Command::new("osascript")
        .arg("-e")
        .arg(format!("tell application \"Music\" to {}", cmd))
        .output()
        .expect("command failed")
}
