use iced::{
    button, scrollable, text_input, window, Button, Color, Column, Container, Element,
    HorizontalAlignment, Length, Row, Sandbox, Scrollable, Settings, Space, Text, TextInput,
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
    Create,
    Submit,
    Cancel,
    ChooseItem(usize),
    DeleteItem(usize),
}

enum RightColumn {
    Editor(TodoEditor),
    Welcome(Welcome),
}

impl Default for RightColumn {
    fn default() -> Self {
        return RightColumn::Welcome(Welcome::default());
    }
}

#[derive(Default)]
struct Todo {
    right_column: RightColumn,
    list: Vec<TodoItem>,
    selected_index: Option<usize>,
    list_scrollable: scrollable::State,
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
        let header = Container::new(
            Text::new("Todo App")
                .width(Length::Fill)
                .size(40)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .padding(20)
        .width(Length::Fill);

        let mut todo_list =
            Scrollable::new(&mut self.list_scrollable).width(Length::FillPortion(1));
        for (i, item) in self.list.iter_mut().enumerate() {
            todo_list =
                todo_list.push(item.view(i, self.selected_index.map_or(false, |index| index == i)));
        }

        let right_column = match &mut self.right_column {
            RightColumn::Editor(editor) => editor.view(),
            RightColumn::Welcome(welcome) => welcome.view(),
        };
        let content = Row::new()
            .push(todo_list)
            .push(right_column)
            .width(Length::FillPortion(2));
        return Column::new().push(header).push(content).into();
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            TodoMessage::InputChange(s) => {
                if let RightColumn::Editor(editor) = &mut self.right_column {
                    editor.update(TodoEditorMessage::InputChange(s));
                }
            }
            TodoMessage::Submit => {
                if let RightColumn::Editor(editor) = &mut self.right_column {
                    if !editor.item.value.is_empty() {
                        let editor = mem::take(editor);
                        let item = editor.item;
                        if let Some(index) = editor.index {
                            self.list[index] = item;
                        } else {
                            self.list.push(item);
                        }
                        self.update(TodoMessage::Cancel);
                    }
                }
            }
            TodoMessage::Cancel => {
                self.right_column = RightColumn::Welcome(Welcome::default());
                self.selected_index = None;
            }
            TodoMessage::Create => {
                let mut editor = TodoEditor::default();
                editor.update(TodoEditorMessage::InputFocus);
                self.right_column = RightColumn::Editor(editor);
                self.selected_index = None;
            }
            TodoMessage::ChooseItem(i) => {
                let mut editor = TodoEditor::from(&self.list[i], i);
                editor.update(TodoEditorMessage::InputFocus);
                self.right_column = RightColumn::Editor(editor);
                self.selected_index = Some(i);
            }
            TodoMessage::DeleteItem(i) => {
                self.list.remove(i);
                self.update(TodoMessage::Cancel);
            }
        }
    }

    fn title(&self) -> String {
        return String::from("hello iced");
    }
}

#[derive(Default)]
struct Welcome {
    create_button: button::State,
}

impl Welcome {
    fn view(&mut self) -> Column<TodoMessage> {
        return Column::new()
            .push(padding_text("Welcome to Todo, an app based on Iced"))
            .push(padding_text("Click a Todo Item on the left to edit"))
            .push(
                Row::new()
                    .push(padding_text("Or add a new item here:"))
                    .push(
                        Button::new(&mut self.create_button, Text::new("Create Item"))
                            .on_press(TodoMessage::Create),
                    ),
            )
            .padding(5)
            .width(Length::FillPortion(2))
            .padding(20);
    }
}

enum TodoEditorMessage {
    InputChange(String),
    InputFocus,
}

#[derive(Debug, Default)]
struct TodoEditor {
    index: Option<usize>,
    item: TodoItem,
    input: text_input::State,
    submit_button: button::State,
    cancel_button: button::State,
    delete_button: button::State,
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
            TodoEditorMessage::InputFocus => {
                self.input.move_cursor_to_end();
                self.input.focus();
            }
        }
    }
    fn view(&mut self) -> Column<TodoMessage> {
        let input = TextInput::new(
            &mut self.input,
            "Todos here",
            &self.item.value,
            TodoMessage::InputChange,
        )
        .padding(2)
        .on_submit(TodoMessage::Submit);
        let submit_button = Button::new(&mut self.submit_button, Text::new("Submit".to_string()))
            .on_press(TodoMessage::Submit);
        let cancel_button = Button::new(&mut self.cancel_button, Text::new("Cancel".to_string()))
            .on_press(TodoMessage::Cancel);
        let text_area =
            Container::new(padding_text("TextArea not support yet")).height(Length::Units(50));
        let mut input_area = Column::new();

        if let Some(index) = self.index {
            input_area = input_area
                .push(
                    Row::new()
                        .push(Space::new(Length::Fill, Length::Shrink))
                        .push(
                            Button::new(&mut self.delete_button, Text::new("Delete Item"))
                                .on_press(TodoMessage::DeleteItem(index)),
                        ),
                )
                .push(Space::new(Length::Fill, Length::Units(5)))
        }
        input_area = input_area
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
        return Container::new(button).width(Length::Fill).into();
    }
}
fn padding_text<'a, Message, T>(label: T) -> Container<'a, Message>
where
    T: Into<String>,
{
    return Container::new(Text::new(label)).padding(5);
}

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
