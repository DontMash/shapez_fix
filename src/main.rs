use std::{
    env::{current_exe, set_current_dir},
    process::Command,
};

fn main() {
    let directory = current_exe().expect("failed to get current folder");
    set_current_dir(directory.parent().unwrap()).expect("failed to set current folder");

    let quarantine_output = Command::new("xattr")
        .arg("-d")
        .arg("-r")
        .arg("com.apple.quarantine")
        .arg(r"shapez 2.app")
        .output()
        .expect("failed to execute quarantine");
    println!("quarantine: {:?}", quarantine_output);

    let apply_output = Command::new("chmod")
        .arg("+x")
        .arg(r"shapez 2.app/Contents/MacOS/shapez 2")
        .output()
        .expect("failed to apply exectuable");
    println!("apply: {:?}", apply_output);
}
