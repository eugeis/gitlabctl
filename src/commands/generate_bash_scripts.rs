// generator.rs
use std::{fs};
use clap::Args;
use crate::ci_parser::{GitlabCi, parse_gitlab_ci};
use crate::gen::{GitCiBashScriptGenerator};

#[derive(Args, Debug)]
pub struct GenerateBashScriptsCommand {
    #[arg(short, long)]
    pub yml_file: String,
    #[arg(short, long, default_value = ".")]
    pub output_dir: String,
}

impl GenerateBashScriptsCommand {
    pub fn run(&self) {
        let yml_content = fs::read_to_string(&self.yml_file)
            .expect("Unable to read gitlab-ci.yml file");

        let gitlab_ci = parse_gitlab_ci(&yml_content);

        GitCiBashScriptGenerator::gen(&gitlab_ci, &self.output_dir);
    }
}

