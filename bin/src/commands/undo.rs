use clap::Args;


#[derive(Args, Debug)]
pub struct UndoArgs {

}

pub fn undo(args: &UndoArgs) {
    println!("Undo: {:?}", args);
}
