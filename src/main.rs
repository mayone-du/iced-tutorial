use iced::{
    button, text_input, Button, Color, Column, Element, Sandbox, Settings, Text, TextInput,
};

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
    double_button: button::State,
    reset_button: button::State,
    text_input: text_input::State,
}

// ユーザーの操作によるメッセージ。イベントの定義。
#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
    DoublePressed,
    ResetPressed,
    TextInputChanged,
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

    // Messageを受け取ったときに実行さ数れる関
    fn update(&mut self, message: Message) {
        // match式で、Messageがマッチするブロックを実行
        match message {
            Message::IncrementPressed => {
                // &mutなselfを受け取っているので、self.valueの値を1増やす
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::DoublePressed => {
                self.value *= 2;
            }
            Message::ResetPressed => {
                self.value = 0;
            }
            Message::TextInputChanged => {
                self.text_input = text_input::State::new();
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
            .push(
                Button::new(&mut self.double_button, Text::new("Double"))
                    .on_press(Message::DoublePressed),
            )
            .push(
                Button::new(
                    &mut self.reset_button,
                    Text::new("Reset").color(Color::from_rgb(100.0, 100.0, 0.0)),
                )
                .on_press(Message::ResetPressed),
            )
            .push(TextInput::new(
                &mut self.value,
                "Enter a number",
                &mut self.value,
                TextInputChanged,
            ))
            .into()
    }
}
