use timecard::Timecard;

use crate::{AppPaths, traits::{Loadable, Saveable}};


pub fn undo(paths: AppPaths) {
    // TODO: expect
    let mut timecard = Timecard::load(&paths.timecard).expect("Failed to load timecard");

    // TODO: expect
    timecard.undo().expect("Failed to undo");
    // TODO: expect
    timecard.save(&paths.timecard).expect("Failed to save Timecard");

    println!("Undo last entry");
}
