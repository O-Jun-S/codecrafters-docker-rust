use std::process::Stdio;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];
    let output = std::process::Command::new(command)
        .args(command_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    let exit_code = output.status.code().unwrap();
    std::process::exit(exit_code);
}

