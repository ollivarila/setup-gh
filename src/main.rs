use anyhow::{anyhow, Result};
use clap::Parser;
use indicatif::ProgressBar;
use std::time::Duration;

mod git_commands;
use git_commands::*;

use crate::error::SetupError;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    origin: String,

    #[arg(
        short,
        long,
        default_value = ".",
        help = "Pathspec used for adding files"
    )]
    pathspec: String,

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
    let setup = SetupGh::with_args(args);
    if let Err(err) = setup.run() {
        setup.clear_bar();
        eprintln!("{}", err);
        std::process::exit(1)
    };
}

struct SetupGh {
    args: Args,
    bar: ProgressBar,
}

impl SetupGh {
    fn with_args(args: Args) -> Self {
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(50));
        Self { args, bar }
    }

    fn run(&self) -> Result<()> {
        let args = &self.args;

        if !args.no_check {
            let is_valid_origin = is_github_origin(&args.origin);

            if !is_valid_origin {
                return err!(SetupError::InvalidOrigin(self.args.origin.clone()));
            }
        }

        self.bar.set_message("Committing file(s)");
        git!("add", &args.pathspec)?;

        git!("commit", "-m", &args.commit_message)?;

        if !args.master {
            self.bar.set_message("Renaming default branch");
            git!("branch", "-M", "main")?;
        }

        self.bar.set_message("Setting up origin");
        git!("remote", "add", "origin", &args.origin)?;

        let branch_name = if args.master { "master" } else { "main" };

        self.bar.set_message("Pushing to remote");
        git!("push", "-u", "origin", branch_name)?;

        self.bar.finish_and_clear();
        Ok(())
    }

    fn clear_bar(&self) {
        self.bar.finish_and_clear();
    }
}

mod error {
    use thiserror::Error;

    #[macro_export]
    macro_rules! err {
        ($e:expr) => {
            Err(anyhow!($e))
        };
    }

    #[derive(Debug, Error)]
    pub enum SetupError {
        #[error("Failed during: git {0} Reason: {1}")]
        CommandFailed(String, String),
        #[error("Invalid origin: {0}")]
        InvalidOrigin(String),
    }
}
