use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{PathBuf, Path}
};
use nums::{
    compiler::{nums_compiler::Compiler, source::Source}
};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
pub struct DNA {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[clap(about, version, author)]
pub enum Command {
    New {
        #[clap(short, long)]
        name: String,
    },

    Build {
        #[clap(short, long, default_value = "~~~!")]
        dir : String
    }
}

impl DNA {
    pub fn execute_sub_command(&self) {
        match &self.command {
            Command::New { name } => {
                todo!()
            }
            Command::Build {dir} => {
                todo!()
            }
        }
    }
}