use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

// 実行時に呼ばれるmain関数
pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

// アプリケーションのstateを定義
#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

// ユーザーの操作によるメッセージ。イベントの定義。
#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    // アプリケーションが実行されるときに呼ばれる関数。自分で明示的に呼ぶひつようはない
    fn new() -> Self {
        Self::default()
    }

    // アプリケーションのタイトルを返す関数
    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    // アプリケーションのUIを描画する関数
    fn view(&mut self) -> Element<Message> {
        // カラム
        Column::new()
            .padding(20)
            // ボタンを生成し、クリックイベントにMessageのIncrementPressedを指定
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            // カウントを表示
            .push(Text::new(self.value.to_string()).size(50))
            // ボタンを生成し、クリックイベントにMessageのDecrementPressedを指定
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
