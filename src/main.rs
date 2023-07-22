mod gitlab;
mod writer;
mod common;

use serde_yaml;
use std::fs;
use clap::Parser;
use crate::writer::YamlModelWriter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Gitlab token
    #[arg(short, long)]
    token: String,

    /// Gitlab URL
    #[arg(short, long,  default_value = "code.siemens.com")]
    url: String,

    /// Gitlab Group to read
    #[arg(short, long)]
    group: String,
}

fn main() {
    let args = Args::parse();

    // Create the client.
    let node = match  gitlab::GroupNodeReader::new(args.url, args.token) {
        Ok(mut reader) => reader.read(&args.group).expect("can't read Gitlab group"),
        Err(err) => panic!("{:?}", err)
    };

    let writer = YamlModelWriter{
        output_dir: "./tmp".to_string(),
        groups_dir: "groups".to_string(),
        model_file_name: ".gitlab".to_string(),
        write_group: true,
        write_model: true,
        handle_children: true,
        model_files: vec!(),
    };

    writer.on_node(&node).expect("can't write model")
}
