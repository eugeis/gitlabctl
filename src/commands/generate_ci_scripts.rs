// generator.rs
use std::{fs};
use clap::Args;
use crate::ci_parser::{parse_gitlab_ci};
use crate::gen::{GitCiScriptGenerator};

#[derive(Args, Debug)]
pub struct GenerateCiScriptsCommand {
    #[arg(short, long)]
    pub yml_file: String,
    #[arg(short, long, default_value = ".")]
    pub output_dir: String,
}

impl GenerateCiScriptsCommand {
    pub fn run(&self) {
        let yml_content = fs::read_to_string(&self.yml_file)
            .expect("Unable to read gitlab-ci.yml file");

        let gitlab_ci = parse_gitlab_ci(&yml_content);

        GitCiScriptGenerator::new(gitlab_ci, &self.output_dir).gen();
    }
}

