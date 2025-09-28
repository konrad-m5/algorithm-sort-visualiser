use eframe::egui;
use rand::Rng;

mod gui;
mod sort;  // Declare the sort module here
use crate::sort::bubble_sort;  // Import from the crate root
use gui::MyApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sorting Visualiser",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
