use std::{env, path, process::Command};

use chrono::Local;

fn main() {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("`git` invocation to succeed");

    let git_hash = String::from_utf8(output.stdout)
        .expect("valid UTF-8 output from `git` invocation");

    println!("cargo::rerun-if-changed=.git/HEAD");
    println!("cargo::rustc-env=GIT_HASH={}", git_hash.trim());

    let timedate_fmt = Local::now().format("%F @ %R");
    let timezone_fmt = iana_time_zone::get_timezone()
        .map(|tz| format!(" ({tz})"))
        .unwrap_or_default();

    println!("cargo::rustc-env=COMPILATION_DATE={timedate_fmt}{timezone_fmt}");

    // trick to get compilation profile
    let profile = env::var("OUT_DIR")
        .expect("OUT_DIR to be set")
        .split(path::MAIN_SEPARATOR)
        .nth_back(3)
        .unwrap_or("unknown")
        .to_string();

    println!("cargo::rustc-env=COMPILATION_PROFILE={profile}");
}
