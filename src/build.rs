#![feature(io,env,path,std_misc)]

use std::old_io::{Command,File};
use std::old_io::process::{InheritFd,Ignored,ProcessOutput};
use std::env;
use std::os::unix::OsStringExt;

fn main() {
    let version = git_describe();
    let timestamp = timestamp();

    let dst = Path::new(env::var("OUT_DIR").expect("Missing environment variable OUT_DIR").into_vec());
    let mut f = File::create(&dst.join("version")).unwrap();
    (writeln!(&mut f, "{} ({})", version, timestamp)).unwrap();
}

fn git_describe() -> String {
    let mut cmd = Command::new("git");
    cmd.arg("describe").arg("--always").arg("--dirty");
    let output = run(&mut cmd);
    assert!(output.status.success());
    let mut output = String::from_utf8(output.output).unwrap();
    while output.ends_with("\n") {
        let len = output.len()-1;
        output.truncate(len);
    }
    assert!(!output.is_empty());
    output
}

fn timestamp() -> String {
    let output = run(Command::new("date").arg("+%F %T %z"));
    assert!(output.status.success());
    chomp(String::from_utf8(output.output).unwrap())
}

fn run(cmd: &mut Command) -> ProcessOutput {
    println!("running {:?}", cmd);
    cmd.stdin(Ignored).stderr(InheritFd(2)).output().unwrap()
}

fn chomp(mut s: String) -> String {
    if s.ends_with("\n") {
        let len = s.len()-1;
        s.truncate(len);
    }
    s
}
