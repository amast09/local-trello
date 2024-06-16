use iced::widget::{column, text_input, Column};
use iced::{Element, Sandbox, Settings};
mod trello_card;

struct TrelloList {
    cards: Vec<trello_card::TrelloCard>,
    next_card_title_input_value: String,
}

#[derive(Debug, Clone)]
enum TrelloListMessage {
    AddTrelloCard,
    NextCardTitleOnInput(String),
}

impl Sandbox for TrelloList {
    type Message = TrelloListMessage;

    fn new() -> Self {
        Self {
            cards: Vec::new(),
            next_card_title_input_value: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Local Trello")
    }

    fn update(&mut self, message: TrelloListMessage) {
        match message {
            TrelloListMessage::AddTrelloCard => {
                self.cards.push(trello_card::TrelloCard {
                    title: self.next_card_title_input_value.clone(),
                    description: String::from(""),
                });
                self.next_card_title_input_value = String::from("");
            }
            TrelloListMessage::NextCardTitleOnInput(text) => {
                self.next_card_title_input_value = text
            }
        }
    }

    fn view(&self) -> Element<TrelloListMessage> {
        let card_elements: Vec<Element<_>> = self
            .cards
            .iter()
            .map(move |card| card.view().into())
            .collect();

        let add_card_input = text_input(
            "Enter a title for this card...",
            &self.next_card_title_input_value,
        )
        .on_input(TrelloListMessage::NextCardTitleOnInput)
        .on_submit(TrelloListMessage::AddTrelloCard);

        let column_of_cards = Column::from_vec(card_elements);

        column![column_of_cards, add_card_input].into()
    }
}

pub fn main() -> iced::Result {
    TrelloList::run(Settings::default())
}
