use iced::{Sandbox, widget::{text_editor, container, TextEditor}, Settings};

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
            content: text_editor::Content::new(),
        }
    }

    fn title(&self) -> String {
        String::from("A cool editor?")
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

        container(input).padding(10).into()
    }
}

