use std::process::Command;

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

    println!(
        "cargo::rustc-env=COMPILATION_DATE={}",
        format_args!(
            "{date} ({tz})",
            date = Local::now().format("%F @ %R"),
            tz = iana_time_zone::get_timezone().unwrap_or_default()
        )
    );
    println!(
        "cargo::rustc-env=COMPILATION_PROFILE={}",
        std::env::var("PROFILE").as_deref().unwrap_or("unknown")
    );
}
