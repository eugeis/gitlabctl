mod gitlab;
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Gitlab token
    #[arg(short, long)]
    token: String,

    /// Gitlab URL
    #[arg(short, long, default_value = "code.siemens.com")]
    url: String,

    /// Gitlab Groups to read
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',')]
    groups: Vec<String>,

    /// Base dir for generation
    #[arg(short, long, default_value = ".")]
    output_dir: String,

    /// write Group Node model to Yaml file
    #[arg(long,default_value_t = false)]
    write_model: bool,

    /// generate Git scripts
    #[arg(long,default_value_t = true)]
    generate_scripts: bool,
}

fn main() {
    let args = Args::parse();

    let handler = if args.write_model && args.generate_scripts {
        Box::new(CompositeHandler {
            handlers: vec![build_yaml_writer(), build_yaml_writer()],
        })
    } else if args.generate_scripts {
        build_script_generator()
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

    match  gitlab::GroupNodeReader::new(args.url, args.token) {
        Ok(mut reader) =>
            for group in &args.groups {
                println!("read group: {:?}", group);
                match reader.read(group) {
                    Ok(node) => fs_handler.on_node(&node).expect("can't write model"),
                    Err(err) => eprintln!("Can't read Gitlab group: {}", err)
                }
            }
        Err(err) => panic!("{:?}", err)
    };
}

fn build_yaml_writer() -> Box<dyn Handler> {
    Box::new(YamlWriter {
        model_file_name: ".gitlab.yaml".to_string(),
        model_files: vec!(),
    })
}

fn build_script_generator() -> Box<dyn Handler> {
    Box::new(Generator{})
}
