use iced::{
    button, container, text_input, window, Button, Color, Column, Container, Element,
    HorizontalAlignment, Length, Row, Sandbox, Settings, Text, TextInput,
};
use std::mem;

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (800, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    };
    Todo::run(settings)
}


#[derive(Clone, Debug)]
enum TodoMessage {
    InputChange(String),
    AddTodo,
}
#[derive(Default)]
struct InputState {
    state: text_input::State,
    value: String,
}

#[derive(Default)]
struct Todo {
    input: InputState,
    button: button::State,
    list: Vec<TodoItem>,
}

impl Sandbox for Todo {
    type Message = TodoMessage;
    fn new() -> Self {
        let mut todo = Self::default();
        todo.list.extend([
            TodoItem::new("hello world".to_string()),
            TodoItem::new("hello iced2".to_string()),
        ]);
        return todo;
    }

    fn view(&mut self) -> Element<Self::Message> {
        let header = Text::new(String::from("hello world"))
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Center);
        // let list: Vec<Element<Self::Message>> = self.list.iter().map(|s| s.view()).collect();
        let todo_list = Container::new(
            Column::with_children(self.list.iter().map(|s| s.view()).collect())
        ).width(Length::FillPortion(1))
        .style(BG);
        let input = TextInput::new(
            &mut self.input.state,
            "Todos here",
            &self.input.value,
            TodoMessage::InputChange,
        )
        // .width(Length::FillPortion(3))
        .on_submit(TodoMessage::AddTodo);

        let input_button = Button::new(&mut self.button, Text::new("Submit".to_string()))
            // .width(Length::FillPortion(3))
            .on_press(TodoMessage::AddTodo);
        let input_area = Column::new()
            .push(input)
            .push(input_button)
            .padding(20)
            .width(Length::FillPortion(2));

        let content = Row::new()
            .push(todo_list)
            .push(input_area)
            .width(Length::FillPortion(2));
        return Column::new().push(header).push(content).into();
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            TodoMessage::InputChange(s) => {
                self.input.value = s;
            }
            TodoMessage::AddTodo => {
                if !self.input.value.is_empty() {
                    let value = mem::take(&mut self.input.value);
                    self.list.push(TodoItem::new(value));
                }
            }
        }
    }

    fn title(&self) -> String {
        return String::from("hello iced");
    }
}

struct TodoItem {
    value: String,
}

impl TodoItem {
    fn new(value: String) -> Self {
        return Self { value };
    }
    fn view<'a>(&self) -> Element<'a, TodoMessage> {
        return Container::new(Text::new(self.value.clone()))
            .padding(10)
            // .style(BG)
            .into();
    }
}
struct BG;

impl container::StyleSheet for BG {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::WHITE),
            background: Color::from_rgb8(249, 40, 20).into(),
            border_color: Color::from_rgb8(229, 20, 0),
            border_width: 5.0,
            border_radius: 8.0,
            ..container::Style::default()
        }
    }
}
