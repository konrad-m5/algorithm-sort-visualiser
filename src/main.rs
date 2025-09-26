use eframe::egui;
use rand::Rng;

mod gui;
use gui::MyApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sorting Visualiser",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
