extern crate time;

use std::io::{Command,File};
use std::io::process::{InheritFd,Ignored,ProcessOutput};
use std::os;

fn main() {
    let version = git_describe();
    let timestamp = timestamp();

    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let mut f = File::create(&dst.join("version")).unwrap();
    (writeln!(f, "{} ({})", version, timestamp)).unwrap();
}

fn git_describe() -> String {
    let mut cmd = Command::new("git");
    cmd.arg("describe").arg("--always").arg("--dirty");
    let output = run(cmd);
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
    time::strftime("%F %T %z", &time::now()).unwrap()
}

fn run(mut cmd: Command) -> ProcessOutput {
    println!("running {}", cmd);
    cmd.stdin(Ignored).stderr(InheritFd(2)).output().unwrap()
}
