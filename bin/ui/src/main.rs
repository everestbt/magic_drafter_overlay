use iced::window::Settings;
use iced::{Element, Theme, Task};
use iced::widget::{
    right_center, text, 
};

pub fn main() -> iced::Result {
    color_eyre::install().expect("Failed to install color eyre");
    let window_settings = Settings{ maximized: true, transparent: true, ..Settings::default() };
    iced::application(App::new, App::update, App::view)
        .window(window_settings)
        .theme(Theme::CatppuccinMocha)
        .transparent(true)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    
}

struct App {
    
}

impl App {
    fn new() -> Self {
        Self {  }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        text("A TEST").into()
    }
}
