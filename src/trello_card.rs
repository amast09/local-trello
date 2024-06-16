use iced::widget::{column, text};
use iced::Element;

#[derive(Debug, Clone)]
pub struct TrelloCard {
    pub title: String,
    pub description: String,
}

impl TrelloCard {
    pub fn view<'a, Message: 'a>(&'a self) -> Element<'a, Message> {
        column![text(self.title.clone()), text(self.description.clone())].into()
    }
}
