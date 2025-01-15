// generator.rs
use crate::ci_parser::parse_gitlab_ci;
use crate::gen::GitlabCiScriptGenerator;
use clap::Args;
use std::fs;

#[derive(Args, Debug)]
pub struct GenerateCiScriptsCommand {
    #[arg(short = 'f', long, default_value = ".gitlab-ci.yml")]
    pub file_ci_yaml: String,
    #[arg(short = 'o', long, default_value = "_scripts")]
    pub output_dir: String,
}

impl GenerateCiScriptsCommand {
    pub fn run(&self) {
        let yml_content = fs::read_to_string(&self.file_ci_yaml).unwrap_or_else(|err| {
            panic!("Unable to read {} file: {}", &self.file_ci_yaml, err);
        });

        let gitlab_ci = parse_gitlab_ci(&yml_content);

        GitlabCiScriptGenerator::new(gitlab_ci, &self.output_dir).gen();
    }
}
