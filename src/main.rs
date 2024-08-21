mod ci_parser;
mod commands;
mod common;
mod gen;
mod gitlab;
mod gitlab_types;
mod handler;
mod yaml;

extern crate lazy_static;
extern crate tera;

use clap::Parser;
use resolve_path::PathResolveExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Gitlab URL
    #[arg(short, long, default_value = "code.siemens.com")]
    url: String,

    /// Gitlab token file for update_scripts.sh path
    #[arg(short, long, default_value = "~/.ssh/gitlab_token_read")]
    token_file: String,

    #[command(subcommand)]
    command: commands::Commands,
}

fn main() {
    let args = Args::parse();
    let gitlab_token_file_path = args.token_file.resolve();
    let gitlab_token: String = std::fs::read_to_string(gitlab_token_file_path)
        .expect("can't read gitlab token file")
        .trim()
        .to_string();

    match &args.command {
        commands::Commands::GenerateGitScripts(cmd) => {
            cmd.run(&args.url, &gitlab_token);
        }
        commands::Commands::ShowPipelines(cmd) => {
            cmd.run(&args.url, &gitlab_token);
        }
        commands::Commands::DeletePipelines(cmd) => {
            cmd.run(&args.url, &gitlab_token);
        }
        commands::Commands::GenerateCiScripts(cmd) => {
            cmd.run();
        }
    }
}
