use clap::Args;

#[derive(Args)]
pub struct Build;

impl Build {
    pub fn run(&self) {
        println!("Building");
    }
}
