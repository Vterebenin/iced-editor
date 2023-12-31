use iced::{Sandbox, widget::{column, row, text_editor, container, TextEditor, text, horizontal_space, Row}, Settings, Length};

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action)
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: text_editor::Content::with(include_str!("main.rs")),
        }
    }

    fn title(&self) -> String {
        String::from("A cool editor")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let input: TextEditor<_, _> = text_editor(&self.content)
            .on_edit(Message::Edit)
            .into();

        let position = {
            let (line, column) = self.content.cursor_position();

            text(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = row![horizontal_space(Length::Fill), position];

        container(column![input, status_bar]).padding(10).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

