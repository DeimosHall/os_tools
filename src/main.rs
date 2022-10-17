use std::process::{Command, Stdio};

fn main() {
    let mut command = Command::new("mkdir");
    println!("{:#?}", command.get_program());

    let output = command
        .arg("hello2")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to create the directory");

    dbg!(&output);
    println!("Directory created: {:#?}", &output.status.success());
    for c in &output.stderr {
        println!("{}", c);
    }
}
