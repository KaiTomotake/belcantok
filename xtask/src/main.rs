#![cfg_attr(not(clippy), allow(warnings))]

mod cli;

use std::process::{Command, Stdio};

use anyhow::Context;
use clap::Parser;
use cli::*;

struct Task {
    name: &'static str,
    args: &'static [&'static str],
}

const TEST: Task = Task {
    name: "test",
    args: &["test", "--locked", "--workspace", "--", "--nocapture"],
};

const CLIPPY: Task = Task {
    name: "clippy",
    args: &[
        "clippy",
        "--tests",
        "--workspace",
        "--locked",
        "--",
        "--deny",
        "warnings",
    ],
};

const FMTCHECK: Task = Task {
    name: "fmt-check",
    args: &["+nightly-2026-10-01", "fmt", "--check"],
};

const FMT: Task = Task {
    name: "fmt",
    args: &["+nightly-2026-10-01", "fmt"],
};

const CHECKS: [Task; 3] = [TEST, CLIPPY, FMTCHECK];

fn cmdline(task: &Task) -> String {
    format!("cargo {}", task.args.join(" "))
}

fn run(task: &Task, out: bool) -> anyhow::Result<()> {
    let cmd = cmdline(task);
    let stdio = || if out { Stdio::inherit() } else { Stdio::null() };
    let status = Command::new("cargo")
        .args(task.args)
        .stdout(stdio())
        .stderr(stdio())
        .status()
        .with_context(|| format!("failed to run {cmd}\n"))?;
    anyhow::ensure!(status.success(), "command failed: {cmd} ({status})\n");
    Ok(())
}

fn verify() -> anyhow::Result<()> {
    let mut failed = Vec::new();
    for task in &CHECKS {
        let result = run(task, false);
        println!(
            "{:<9} : {}",
            task.name,
            if result.is_ok() { "✓" } else { "✗" }
        );
        if let Err(err) = result {
            failed.push((task.name, err));
        }
    }
    if !failed.is_empty() {
        println!();
    }
    for (name, err) in &failed {
        eprintln!("===== {name} =====\n{err:#}");
    }
    anyhow::ensure!(failed.is_empty(), "{} failed\n", failed.len());
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        SubCommands::Test => run(&TEST, true),
        SubCommands::Clippy => run(&CLIPPY, true),
        SubCommands::FmtCheck => run(&FMTCHECK, true),
        SubCommands::Fmt => run(&FMT, true),
        SubCommands::Verify => verify(),
    }
}
