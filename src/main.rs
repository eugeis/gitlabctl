mod gitlab;

use serde::{Serialize, Deserialize};
use serde_yaml;
use std::fs;
use std::collections::HashSet;
use std::path::Path;
use log::{info, warn};
use thiserror::Error;
use clap::Parser;

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
    let group_node = match  gitlab::GroupNodeReader::new(args.url, args.token) {
        Ok(mut reader) => reader.read(&args.group).expect("can't read Gitlab group"),
        Err(err) => panic!("{:?}", err)
    };

    let group_node_yaml = serde_yaml::to_string(&group_node).expect("Can't marshal yaml");
    let file_name = format!("{:?}.yaml", group_node.group.id.value());
    fs::write(&file_name, group_node_yaml).expect("Unable to write file");
    println!("File {:?} written", file_name)
}
