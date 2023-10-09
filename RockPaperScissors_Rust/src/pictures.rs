use eframe::{egui, egui::{Vec2, TextureId}};

#[derive(Default)]
pub struct RockPicture{
    texture: Option<(Vec2, TextureId)>
}

impl RockPicture {
    fn load(&mut self){
        //
        let image_data = include_bytes!("./../pictures/rock.jpg");
        
    }

}

