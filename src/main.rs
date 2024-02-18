use anyhow::{anyhow, Result};
use clap::{arg, Parser};
use regex::Regex;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    origin: String,

    #[arg(
        short,
        long,
        default_value = ".",
        help = "What file to commit in the initial commit"
    )]
    file_to_commit: String,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "Keep default branch name as master"
    )]
    master: bool,

    #[arg(short, long, default_value = "init", help = "Initial commit message")]
    commit_message: String,

    #[arg(
        long,
        default_value = "false",
        help = "Skip validation for origin string"
    )]
    no_check: bool,
}

fn main() {
    let args = Args::parse();
    if let Err(err) = setup_gh(args) {
        eprintln!("{}", err);
        std::process::exit(1)
    };
}

fn setup_gh(args: Args) -> anyhow::Result<()> {
    if !args.no_check {
        let is_valid_origin = is_github_origin(&args.origin);

        if !is_valid_origin {
            return Err(anyhow!(SetupError::InvalidOrigin(args.origin)));
        }
    }

    git_add(&args.file_to_commit)?;

    git_commit(&args.commit_message)?;

    if !args.master {
        rename_master_to_main()?;
    }

    git_remote_add_origin(&args.origin)?;

    let branch_name = if args.master { "master" } else { "main" };

    git_push_upstream(branch_name)?;
    Ok(())
}

fn git_add(files: &str) -> Result<()> {
    git(["add", files])
}

fn git_commit(msg: &str) -> Result<()> {
    git(["commit", "-m", &msg])
}

fn rename_master_to_main() -> Result<()> {
    git(["branch", "-M", "main"])
}

fn git_remote_add_origin(origin: &str) -> Result<()> {
    git(["remote", "add", "origin", origin])
}

fn git_push_upstream(branch_name: &str) -> Result<()> {
    git(["push", "-u", "origin", branch_name])
}

#[derive(Debug)]
enum SetupError {
    CommandFailed(String, String),
    InvalidOrigin(String),
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandFailed(sub_cmd, reason) => {
                write!(f, "Failed during: git {sub_cmd}, Reason: {reason}")
            }
            Self::InvalidOrigin(origin) => write!(f, "Invalid origin: {origin}"),
        }
    }
}

fn git<I, S>(args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<std::ffi::OsStr>,
{
    let out = Command::new("git").args(args.clone()).output()?;

    if !out.status.success() {
        let errors = String::from_utf8(out.stderr)?;
        let sub_cmd = args.into_iter().next().unwrap();
        let sub_cmd = sub_cmd.as_ref().to_str().unwrap().to_string();

        Err(anyhow!(SetupError::CommandFailed(sub_cmd, errors)))
    } else {
        Ok(())
    }
}

fn is_github_origin(origin: &str) -> bool {
    let r = Regex::new(r"git@github.com:[A-z\d-]+\/[A-z\d-]+.git").unwrap();
    r.is_match(origin)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_github_origin() {
        let origin = "git@github.com:test-account-123/test-repo-123.git";
        let res = is_github_origin(origin);
        assert!(res);
        let origin = "not-real";
        let res = is_github_origin(origin);
        assert!(res == false)
    }
}
