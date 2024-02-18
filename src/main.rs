use anyhow::{anyhow, Result};
use clap::{arg, Parser};
use indicatif::ProgressBar;
use std::time::Duration;

mod git_commands;
use git_commands::*;

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
                return Err(anyhow!(SetupError::InvalidOrigin(args.origin.clone())));
            }
        }

        self.bar.set_message("Committing file(s)");
        git_add(&args.pathspec)?;

        git_commit(&args.commit_message)?;

        if !args.master {
            self.bar.set_message("Renaming default branch");
            rename_master_to_main()?;
        }

        self.bar.set_message("Setting up origin");
        git_remote_add_origin(&args.origin)?;

        let branch_name = if args.master { "master" } else { "main" };

        self.bar.set_message("Pushing to remote");
        git_push_upstream(branch_name)?;
        Ok(())
    }

    fn clear_bar(&self) {
        self.bar.finish_and_clear();
    }
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
