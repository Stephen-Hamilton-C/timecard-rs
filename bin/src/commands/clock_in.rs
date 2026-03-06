use clap::Args;


#[derive(Args, Debug)]
pub struct InArgs {

}

pub fn clock_in(args: &InArgs) {
    println!("Clock in: {:?}", args);
}
