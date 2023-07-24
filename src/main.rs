mod gitlab;
mod handler;
mod common;
mod gen;
mod yaml;

#[macro_use]
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

    /// Gitlab Group to read
    #[arg(short, long)]
    group: String,

    /// Base dir for generation
    #[arg(short, long, default_value = ".")]
    output_dir: String,

    /// relative dir for Gitlab group models
    #[arg(long, default_value = "groups")]
    groups_dir: String,

    /// write Group Node model to Yaml file
    #[arg(long,default_value_t = false)]
    write_model: bool,

    /// generate Git scripts
    #[arg(long,default_value_t = true)]
    generate_scripts: bool,
}

fn main() {
    let args = Args::parse();

    // Create the client.
    let node = match  gitlab::GroupNodeReader::new(args.url, args.token) {
        Ok(mut reader) => reader.read(&args.group).
            expect("can't read Gitlab group"),
        Err(err) => panic!("{:?}", err)
    };

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

    let writer = FsModelHandler {
        output_dir: args.output_dir,
        handle_children: true,
        handler,
    };

    writer.on_node(&node).expect("can't write model")
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
