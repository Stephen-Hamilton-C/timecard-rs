use clap::Args;


#[derive(Args, Debug)]
pub struct CleanArgs {

}

pub fn clean(args: &CleanArgs) {
    println!("Clean: {:?}", args);
}
