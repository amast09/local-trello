use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings};

struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum CounterMessage {
    Decrement,
    Increment,
}

impl Sandbox for Counter {
    type Message = CounterMessage;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: CounterMessage) {
        match message {
            CounterMessage::Decrement => {
                self.value -= 1;
            }
            CounterMessage::Increment => {
                self.value += 1;
            }
        }
    }

    fn view(&self) -> Element<CounterMessage> {
        column![
            button("+").on_press(CounterMessage::Increment),
            text(self.value),
            button("-").on_press(CounterMessage::Decrement),
        ].into()
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}
