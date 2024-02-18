use std::process::Command;

use anyhow::{anyhow, Result};
use regex::Regex;

use crate::SetupError;

pub fn git_add(files: &str) -> Result<()> {
    git(["add", files])
}

pub fn git_commit(msg: &str) -> Result<()> {
    git(["commit", "-m", &msg])
}

pub fn rename_master_to_main() -> Result<()> {
    git(["branch", "-M", "main"])
}

pub fn git_remote_add_origin(origin: &str) -> Result<()> {
    git(["remote", "add", "origin", origin])
}

pub fn git_push_upstream(branch_name: &str) -> Result<()> {
    git(["push", "-u", "origin", branch_name])
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

pub fn is_github_origin(origin: &str) -> bool {
    let r = Regex::new(r"git@github.com:[A-z\d-]+\/[A-z\d-]+.git").unwrap();
    let Some(cap) = r.captures(origin) else {
        return false;
    };
    let cap = cap.get(0).unwrap();
    cap.as_str() == origin
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
        assert!(res == false);
        let origin = "asdfgit@github.com:test-account-123/test-repo-123.gitasdf";
        let res = is_github_origin(origin);
        assert!(res == false);
    }
}
