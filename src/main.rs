mod gitlab;
mod gitlab_types;
mod handler;
mod common;
mod gen;
mod yaml;

extern crate tera;
#[macro_use]
extern crate lazy_static;

use clap::Parser;
use crate::handler::{FsModelHandler, CompositeHandler, Handler};
use crate::yaml::YamlWriter;
use crate::gen::Generator;
use std::boxed::Box;
use std::fs;
use resolve_path::PathResolveExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Gitlab URL
    #[arg(short, long, default_value = "code.siemens.com")]
    url: String,

    /// Gitlab Groups to read
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',', required = true)]
    groups: Vec<String>,

    /// Base dir for generation
    #[arg(short, long, default_value = ".")]
    output_dir: String,

    /// Gitlab token file for update_scripts.sh path
    #[arg(long, default_value = "~/.ssh/gitlab_token_read")]
    gitlab_token_file: String,

    /// write Group Node model to Yaml file
    #[arg(long,default_value_t = false)]
    write_model: bool,

    /// generate Git scripts
    #[arg(long,default_value_t = true)]
    generate_scripts: bool,
}

fn main() {
    let args = Args::parse();
    let gitlab_token_file_path = args.gitlab_token_file.resolve();
    let gitlab_token: String = fs::read_to_string(gitlab_token_file_path).expect("can't read gitlab token file").trim().to_string();
    let handler = if args.write_model && args.generate_scripts {
        Box::new(CompositeHandler {
            handlers: vec![build_yaml_writer(), build_script_generator(args.gitlab_token_file)],
        })
    } else if args.generate_scripts {
        build_script_generator(args.gitlab_token_file)
    } else if args.write_model {
        build_yaml_writer()
    } else {
        build_yaml_writer()
    };

    let fs_handler = FsModelHandler {
        output_dir: args.output_dir,
        handle_children: true,
        handler,
    };

    match  gitlab::GroupNodeReader::new(args.url, gitlab_token) {
        Ok(mut reader) =>
            for group in &args.groups {
                println!("read group: {:?}", group);
                match reader.read(group) {
                    Ok(node) => {
                        fs_handler.on_node(&node).expect("can't write model")
                    },
                    Err(err) => println!("can't read the group '{}': {}", group, err)
                }
            }
        Err(err) => panic!("can't connect to Gitlab: {}", err)
    };
}

fn build_yaml_writer() -> Box<dyn Handler> {
    Box::new(YamlWriter {
        model_file_name: ".gitlab.yaml".to_string(),
        model_files: vec!(),
    })
}

fn build_script_generator(gitlab_token_file: String) -> Box<dyn Handler> {
    Box::new(Generator{gitlab_token_file: gitlab_token_file})
}
