use iced::{Element, Sandbox, Settings};

fn main() -> Result<(), iced::Error> {
    ShaiHulud::run(Settings::default())
}

struct ShaiHulud {}

#[derive(Debug)]
enum Message {}

impl iced::Sandbox for ShaiHulud {
    type Message = Message;

    fn new() -> Self {
        ShaiHulud {}
    }

    fn title(&self) -> String {
        String::from("Shai-Hulud")
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> Element<'_, Self::Message> {
        iced::widget::text("Hello, world!").into()
    }
}
