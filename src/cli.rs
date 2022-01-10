use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
pub struct DNA {
    #[clap(subcommand)]
    command : Command

}


#[derive(Subcommand)]
#[clap(about, version, author)]
pub enum Command {
    Init {
        #[clap(short, long)]
        y: bool
    }
}

impl DNA {
    pub fn execute_sub_command(&self) {
        match &self.command {
            Command::Init { y } =>  {
                if *y {

                } else {

                }
            }
        }
    }
}