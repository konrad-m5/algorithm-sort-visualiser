use eframe::egui;
use rodio::{OutputStream, Sink, source::SineWave};
use rand::Rng;
use std::time::{Duration, Instant};
use crate::sort::{bubble_sort, selection_sort, insertion_sort};

// Struct for color settings
pub struct drawColor{
    bar:egui::Color32,
    background:egui::Color32,
    highlight:egui::Color32,
    sorted:egui::Color32,
    bar_width:f32,
    max_bar_height:f32,

}

// Main application struct
pub struct MyApp{
    list: Vec<i32>, // The list to be sorted
    colors: drawColor, // Color setting
    is_sorting: bool, // Is the sorting process active
    current_step: usize, // Current step in the sorting process
    comparing_indices: Vec<usize>, // Indices currently being compared
    current_algorithm: SortAlgorithm, // Currently selected sorting algorithm
    sorting_steps: Vec<SortingStep>,  // Pre-generated animation steps
    sorted_indices: Vec<bool>,        // Track which bars are sorted
    animation_speed: Duration,       // Control animation speed
    last_update: Instant,            // For timing animation
    speed_multiplier: u32,           // Speed multiplier for ultra-fast mode
}

// Enum for sorting algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortAlgorithm{
    BubbleSort,
    InsertionSort,
    SelectionSort,
}

// Enum for animation steps
#[derive(Debug, Clone)]
pub enum SortingStep {
    Compare(usize, usize),  // Compare two indices
    Swap(usize, usize),     // Swap two indices
    SetSorted(usize),       // Mark index as sorted
    Finished,               // Animation complete
}

// Implement Default trait for MyApp
impl Default for drawColor{
    fn default() -> Self{
        Self{
            bar:egui::Color32::from_rgb(0, 255, 0),
            background:egui::Color32::from_rgb(0, 0, 0),
            highlight:egui::Color32::from_rgb(255, 0, 0),
            sorted:egui::Color32::from_rgb(0, 0, 255),
            bar_width: 7.0,
            max_bar_height: 300.0,
        
        }// End Self

    }// End fn default

}// End impl Default for drawColor


// Implement Default trait for MyApp
impl Default for MyApp {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let size = 100;
        let min_value = 20;
        let max_value = 500;
        
        // Generate random list on startup
        let list: Vec<i32> = (0..size).map(|_| rng.gen_range(min_value..=max_value)).collect();
        let sorted_indices = vec![false; list.len()];
        
        Self {
            list,
            colors: drawColor::default(),
            is_sorting: false,
            current_step: 0,
            comparing_indices: Vec::new(),
            current_algorithm: SortAlgorithm::BubbleSort,
            sorting_steps: Vec::new(),
            sorted_indices,
            animation_speed: Duration::from_millis(0), // Instant speed
            last_update: Instant::now(),
            speed_multiplier: 10,                    // Process 10 steps per frame

        }
    }
}

impl MyApp{

    // Constructor method
    fn new() -> Self{
        Self::default()
    }

    // Generate random list of integers 
    fn generate_random_list(&mut self, size: usize, max_value: i32) {

        let mut rng = rand::thread_rng();
        let min_value = 20;
        // Make a list up to the given size with random values between min_value and max_value
        self.list = (0..size).map(|_| rng.gen_range(min_value..=max_value)).collect();
        self.is_sorting = false;
        self.current_step = 0;
        self.comparing_indices.clear();
        self.sorting_steps.clear();
        self.sorted_indices = vec![false; size];

    }// End fn generate_random_list

    fn generate_bars(&self, ui: &mut egui::Ui) {
        if self.list.is_empty() { return; }
    
        // Bar dimensions
        let bar_width = self.colors.bar_width;
        let max_height = self.colors.max_bar_height;
        let max_value = *self.list.iter().max().unwrap_or(&1) as f32;
    
        // Calculate total width needed
        let total_width = (self.list.len() as f32) * bar_width;
        let available_width = ui.available_width();
    
        // Center the bars horizontally
        ui.allocate_ui_with_layout(
            egui::Vec2::new(available_width, max_height),

            // Center layout
            egui::Layout::top_down(egui::Align::Center),
            |ui| {

                // Create a custom painting area
                let (response, painter) = ui.allocate_painter(
                    egui::Vec2::new(total_width, max_height),
                    egui::Sense::hover()
                );
            
            
                let rect = response.rect;
                let baseline_y = rect.bottom(); // Common baseline at bottom
            
                // Draw each bar from the baseline upward
                // Enumerate used to get index and value of the list
                for (i, &value) in self.list.iter().enumerate() {
                    let bar_height = (value as f32 / max_value) * max_height;
                    let x = rect.left() + (i as f32 * bar_width);
                
                    // Determine color based on state
                    let color = if self.comparing_indices.contains(&i) {
                        self.colors.highlight  // RED - being compared
                    } else if i < self.sorted_indices.len() && self.sorted_indices[i] {
                        self.colors.sorted     // BLUE - sorted
                    } else {
                        self.colors.bar        // GREEN - unsorted
                    };// End if else
                
                    // Create rectangle from baseline upward
                    let bar_rect = egui::Rect::from_min_size(
                        egui::pos2(x, baseline_y - bar_height), // Start from baseline, go up
                        egui::vec2(bar_width - 1.0, bar_height) // -1.0 for spacing
                    );
                
                    painter.rect_filled(bar_rect, 0.0, color);

                }// End for loop

            }//End of ui topdown

        );// End of ui.allocate_ui_with_layout

    }// End fn generate_bars

    // Animation update method using steps
    fn update_animation(&mut self, ctx: &egui::Context) {
        if !self.is_sorting || self.current_step >= self.sorting_steps.len() {
            return;
        }

        // Check if enough time has passed
        if self.last_update.elapsed() >= self.animation_speed {
            // Process multiple steps per frame for ultra-fast animation
            let steps_per_frame = self.speed_multiplier; // Use dynamic speed multiplier
            
            for _ in 0..steps_per_frame {
                if self.current_step >= self.sorting_steps.len() {
                    break;
                }
                
                // Get current step
                if let Some(step) = self.sorting_steps.get(self.current_step) {
                    match step {
                        
                        SortingStep::Compare(i, j) => {
                            self.comparing_indices = vec![*i, *j];
                        }
                        SortingStep::Swap(i, j) => {
                            self.list.swap(*i, *j);
                            self.comparing_indices = vec![*i, *j];
                        }
                        SortingStep::SetSorted(i) => {
                            if *i < self.sorted_indices.len() {
                                self.sorted_indices[*i] = true;
                            }
                            self.comparing_indices.clear();
                        }
                        SortingStep::Finished => {
                            self.is_sorting = false;
                            self.comparing_indices.clear();
                            for i in 0..self.sorted_indices.len() {
                                self.sorted_indices[i] = true;
                            }
                            return; // Exit early when finished
                        }

                    }// End match

                }// end inner if let
                
                self.current_step += 1;
            }
            
            self.last_update = Instant::now();
            ctx.request_repaint();

        }// End if 
        ctx.request_repaint();
    }// End fn update_animation



    // Dropdown menu for selecting sorting algorithm
    fn drop_down_menu(&mut self, ui: &mut egui::Ui) {
        let mut selected_algorithm = self.current_algorithm;
        egui::ComboBox::from_label("Select Algorithm")
            .selected_text(format!("{:?}", selected_algorithm))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected_algorithm, SortAlgorithm::BubbleSort, "Bubble Sort");
                ui.selectable_value(&mut selected_algorithm, SortAlgorithm::InsertionSort, "Insertion Sort");
                ui.selectable_value(&mut selected_algorithm, SortAlgorithm::SelectionSort, "Selection Sort");
            });

            if selected_algorithm != self.current_algorithm {
                self.current_algorithm = selected_algorithm;
            }

    }// End fn drop_down_menu

    // Generate sorting steps based on selected algorithm
    fn generate_sorting_steps(&mut self) -> Vec<SortingStep> {
        let mut list_copy = self.list.clone();
        match self.current_algorithm {
            SortAlgorithm::BubbleSort => bubble_sort(&mut list_copy),
            SortAlgorithm::InsertionSort => {insertion_sort(&mut list_copy)}
            SortAlgorithm::SelectionSort => {selection_sort(&mut list_copy)}
        }
    }// End fn generate_sorting_steps

    fn stop_button(&mut self){
        self.is_sorting = false;
        self.current_step = 0;
        self.comparing_indices.clear();
        self.sorted_indices = vec![false; self.list.len()];
    }


} // End impl MyApp


// Implement eframe::App trait for MyApp
impl eframe::App for MyApp{
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        // animation update
        self.update_animation(ctx);
    
        egui::CentralPanel::default().show(ctx, |ui| {

            // Top panl with buttons and dropdown menu
            ui.horizontal(|ui| {

                if ui.button("Generate Random List").clicked(){
                    self.generate_random_list(100, 500);
                    self.sorting_steps.clear();
                    self.sorted_indices = vec![false; self.list.len()];
                }// end if

                if ui.button("Start Sorting").clicked(){
                    self.is_sorting = true;
                    self.current_step = 0;
                    self.comparing_indices.clear();
                    // Generate sorting steps based on algorithm
                    self.sorting_steps = self.generate_sorting_steps();
                    self.current_algorithm = self.current_algorithm;
                }// end if 

                if ui.button("Stop Sorting").clicked(){
                    self.stop_button();
                }// end if

                // Dropdown menu for selecting sorting algorithm
                self.drop_down_menu(ui);

                // Speed control slider
                ui.label("Speed:");
                ui.add(egui::Slider::new(&mut self.speed_multiplier, 1..=100).text("steps/frame"));

            });// end ui.horizontal
            

            // Some padding
            ui.add_space(200.0);
            self.generate_bars(ui);

        });
    
    }// End of fn update

}// End impl eframe::App for MyApp
