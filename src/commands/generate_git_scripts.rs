use crate::gen::GitScriptGenerator;
use crate::gitlab::GroupNodeReader;
use crate::handler::{FsModelHandler, Handler};
use clap::Args;

#[derive(Args, Debug)]
pub struct GenerateGitScriptsCommand {
    /// Gitlab Groups to read
    #[arg(short = 'g', long, value_parser, num_args = 1.., value_delimiter = ',', required = true)]
    pub groups: Vec<String>,

    /// Base dir for generation
    #[arg(short = 'o', long, default_value = ".")]
    pub output_dir: String,

    /// Generate Git scripts
    #[arg(long="templates", default_value = "clone.sh,pull.sh,clone-recursive.sh,pull-recursive.sh,scripts.sh", value_delimiter = ',', num_args = 1..)]
    pub generate_templates: Vec<String>,
}

impl GenerateGitScriptsCommand {
    pub fn run(&self, url: &str, gitlab_token_file: &str, gitlab_token: &str) {
        let handler = build_script_generator(
            self.generate_templates.clone(),
            gitlab_token_file.to_string(),
        );

        let fs_handler = FsModelHandler {
            output_dir: self.output_dir.clone(),
            handle_children: true,
            handler,
        };

        match GroupNodeReader::new(url, gitlab_token) {
            Ok(mut reader) => {
                for group in &self.groups {
                    println!("read group: {:?}", group);
                    match reader.read(group) {
                        Ok(node) => fs_handler.on_node(&node).expect("can't write model"),
                        Err(err) => println!("can't read the group '{}': {}", group, err),
                    }
                }
            }
            Err(err) => panic!("can't connect to Gitlab: {}", err),
        };
    }
}

fn build_script_generator(templates: Vec<String>, gitlab_token_file: String) -> Box<dyn Handler> {
    Box::new(GitScriptGenerator {
        templates,
        gitlab_token_file,
    })
}
