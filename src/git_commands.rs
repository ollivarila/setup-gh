use crate::{err, error::SetupError};
use anyhow::anyhow;
use regex::Regex;
use std::process::Command;

#[macro_export]
macro_rules! git {
    ($($args:expr),*) => {
        git([$($args),*])
    };
}

pub fn git<const N: usize>(args: [&str; N]) -> anyhow::Result<()> {
    let sub_cmd = args[0].to_string();

    let out = Command::new("git").args(args.clone()).output()?;

    if !out.status.success() {
        let errors = String::from_utf8(out.stderr)?;
        err!(SetupError::CommandFailed(sub_cmd, errors))
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
