//using statements
use eframe::{
    egui,
    egui::{vec2, Image, Ui},
};

/// Function for drawing a rock image in the window.
/// 
/// Takes in: 
/// - ui = Ui egui struct
pub fn show_rock_picture(ui: &mut Ui) {
    ui.add_sized(
        vec2(110., 110.),
        Image::new(egui::include_image!("../pictures/rock.jpg")).rounding(10.),
    );
}

/// Function for drawing a paper image in the window.
/// 
/// Takes in: 
/// - ui = Ui egui struct
pub fn show_paper_picture(ui: &mut Ui) {
    ui.add_sized(
        vec2(100., 100.),
        Image::new(egui::include_image!("../pictures/paper.jpg")).rounding(10.),
    );
}

/// Function for drawing a scissors image in the window.
/// 
/// Takes in: 
/// - ui = Ui egui struct
pub fn show_scissors_picture(ui: &mut Ui) {
    ui.add_sized(
        vec2(110., 110.),
        Image::new(egui::include_image!("../pictures/scissors.jpg")).rounding(10.),
    );
}
