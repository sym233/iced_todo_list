use iced::{
    button, text_input, window, Button, Color, Column, Container, Element, HorizontalAlignment,
    Length, Row, Sandbox, Settings, Space, Text, TextInput,
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
    Submit,
    Cancel,
    ChooseItem(usize),
}

#[derive(Debug, Default)]
struct Todo {
    todo_editor: TodoEditor,
    list: Vec<TodoItem>,
    selected_index: Option<usize>,
}

impl Sandbox for Todo {
    type Message = TodoMessage;
    fn new() -> Self {
        let mut todo = Self::default();
        todo.list.extend([
            TodoItem::from("hello world".to_string()),
            TodoItem::from("hello iced2".to_string()),
        ]);
        return todo;
    }

    fn view(&mut self) -> Element<Self::Message> {
        let header = Text::new(String::from("hello world"))
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Center);
        // let list: Vec<Element<Self::Message>> = self.list.iter().map(|s| s.view()).collect();
        let todo_list = Container::new(Column::with_children(
            self.list
                .iter_mut()
                .enumerate()
                .map(|(i, s)| s.view(i, self.selected_index.map_or(false, |index| index == i)))
                .collect(),
        ))
        .width(Length::FillPortion(1));
        // .style(BG);
        let content = Row::new()
            .push(todo_list)
            .push(self.todo_editor.view())
            .width(Length::FillPortion(2));
        return Column::new().push(header).push(content).into();
    }

    fn update(&mut self, message: Self::Message) {
        // println!("{:?}", message);
        match message {
            TodoMessage::InputChange(s) => {
                self.todo_editor.update(TodoEditorMessage::InputChange(s));
            }
            TodoMessage::Submit => {
                if !self.todo_editor.item.value.is_empty() {
                    let todo_editor = mem::take(&mut self.todo_editor);
                    let item = todo_editor.item;
                    if let Some(index) = todo_editor.index {
                        self.list[index] = item;
                    } else {
                        self.list.push(item);
                    }
                    self.selected_index = None;
                }
            }
            TodoMessage::Cancel => {
                self.todo_editor = TodoEditor::default();
                self.selected_index = None;
            }
            TodoMessage::ChooseItem(i) => {
                self.todo_editor = TodoEditor::from(&self.list[i], i);
                self.selected_index = Some(i);
            }
        }
    }

    fn title(&self) -> String {
        return String::from("hello iced");
    }
}

enum TodoEditorMessage {
    InputChange(String),
    // SubmitChange,
}

#[derive(Debug, Default)]
struct TodoEditor {
    index: Option<usize>,
    item: TodoItem,
    input: text_input::State,
    submit_button: button::State,
    cancel_button: button::State,
}

impl TodoEditor {
    fn from(todo_item: &TodoItem, index: usize) -> Self {
        return Self {
            item: todo_item.clone(),
            index: Some(index),
            ..Self::default()
        };
    }
    fn update(&mut self, message: TodoEditorMessage) {
        match message {
            TodoEditorMessage::InputChange(s) => self.item.value = s,
        }
    }
    fn view(&mut self) -> Column<TodoMessage> {
        let input = TextInput::new(
            &mut self.input,
            "Todos here",
            &self.item.value,
            TodoMessage::InputChange,
        )
        // .width(Length::FillPortion(3))
        .on_submit(TodoMessage::Submit);
        let submit_button = Button::new(&mut self.submit_button, Text::new("Submit".to_string()))
            // .width(Length::FillPortion(3))
            .on_press(TodoMessage::Submit);
        let cancel_button = Button::new(&mut self.cancel_button, Text::new("Cancel".to_string()))
            .on_press(TodoMessage::Cancel);
        let text_area =
            Container::new(Text::new("TextArea not support yet")).height(Length::Units(50));
        let input_area = Column::new()
            .push(input)
            .push(text_area)
            .push(
                Row::new()
                    .push(Space::new(Length::Fill, Length::Units(10)))
                    .push(cancel_button)
                    .push(submit_button),
            )
            .padding(20)
            .width(Length::FillPortion(2));
        return input_area;
    }
}

#[derive(Clone, Debug, Default)]
struct TodoItem {
    // index: Option<usize>,
    button: button::State,
    value: String,
}

impl TodoItem {
    fn from(value: String) -> Self {
        return Self {
            value,
            ..Self::default()
        };
    }
    fn view(&mut self, index: usize, is_selected: bool) -> Element<TodoMessage> {
        let button = Button::new(&mut self.button, Text::new(self.value.clone()))
            .padding(10)
            .style(if is_selected {
                ListItem::Selected
            } else {
                ListItem::Unselected
            })
            .on_press(TodoMessage::ChooseItem(index))
            .width(Length::Fill);
        return Container::new(button)
            .width(Length::Fill)
            // .style(BG)
            .into();
    }
}
// struct BG;

// impl container::StyleSheet for BG {
//     fn style(&self) -> container::Style {
//         container::Style {
//             text_color: Some(Color::WHITE),
//             background: Color::from_rgb8(249, 40, 20).into(),
//             border_color: Color::from_rgb8(229, 20, 0),
//             border_width: 5.0,
//             border_radius: 8.0,
//             ..container::Style::default()
//         }
//     }
// }

#[derive(Debug)]
enum ListItem {
    Selected,
    Unselected,
}

impl ListItem {
    fn default() -> button::Style {
        return button::Style {
            background: Color::from_rgba8(249, 40, 20, 0.2).into(),
            ..button::Style::default()
        };
    }
}

impl button::StyleSheet for ListItem {
    fn active(&self) -> button::Style {
        return match self {
            ListItem::Selected => button::Style {
                background: Color::from_rgba8(249, 40, 20, 0.7).into(),
                ..Self::default()
            },
            ListItem::Unselected => Self::default(),
        };
    }
    fn hovered(&self) -> button::Style {
        return match self {
            ListItem::Selected => button::Style {
                background: Color::from_rgba8(200, 30, 10, 0.7).into(),
                ..Self::default()
            },
            ListItem::Unselected => button::Style {
                background: Color::from_rgba8(200, 30, 10, 0.2).into(),
                ..Self::default()
            },
        };
    }
    fn pressed(&self) -> button::Style {
        return match self {
            ListItem::Selected => button::Style {
                background: Color::from_rgba8(180, 20, 10, 0.7).into(),
                ..Self::default()
            },
            ListItem::Unselected => button::Style {
                background: Color::from_rgba8(180, 20, 10, 0.2).into(),
                ..Self::default()
            },
        };
    }
}
