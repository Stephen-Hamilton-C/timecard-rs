use clap::Args;


#[derive(Args, Debug)]
pub struct LogArgs {

}

pub fn log(args: &LogArgs) {
    println!("Log: {:?}", args);
}
