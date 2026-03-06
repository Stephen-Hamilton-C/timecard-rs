use clap::Args;


#[derive(Args, Debug)]
pub struct OutArgs {

}

pub fn clock_out(args: &OutArgs) {
    println!("Clock out: {:?}", args);
}
