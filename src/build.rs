#![feature(std_misc)]

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::os::unix::prelude::*;
use std::path::PathBuf;
use std::process::{self,Command,Stdio};

fn main() {
    let version = git_describe();
    let timestamp = timestamp();

    let out_dir = env::var_os("OUT_DIR").expect("Missing environment variable OUT_DIR").into_vec();
    let mut dst = PathBuf::new(&<OsString as OsStringExt>::from_vec(out_dir));
    dst.push("version");
    let mut f = File::create(&dst).unwrap();
    (writeln!(&mut f, "{} ({})", version, timestamp)).unwrap();
}

fn git_describe() -> String {
    let output = run(Command::new("git").args(&["describe", "--always", "--dirty"]));
    assert!(output.status.success());
    let output = chomp(String::from_utf8(output.stdout).unwrap());
    assert!(!output.is_empty());
    output
}

fn timestamp() -> String {
    let output = run(Command::new("date").arg("+%F %T %z"));
    assert!(output.status.success());
    chomp(String::from_utf8(output.stdout).unwrap())
}

fn run(cmd: &mut Command) -> process::Output {
    println!("running {:?}", cmd);
    cmd.stdin(Stdio::null()).stderr(Stdio::inherit()).output().unwrap()
}

fn chomp(mut s: String) -> String {
    if s.ends_with("\n") {
        let len = s.len()-1;
        s.truncate(len);
    }
    s
}
