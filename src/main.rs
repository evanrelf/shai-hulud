use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc};

fn main() {
    let main_window =
        WindowDesc::new(build_root_widget()).title(LocalizedString::new("Shai-Hulud"));

    let initial_state = State {
        text: String::from("Hello, world!"),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .unwrap();
}

#[derive(Clone, Data, Lens)]
struct State {
    text: String,
}

fn build_root_widget() -> impl Widget<State> {
    use druid::widget::Label;

    Label::new(|data: &State, _env: &Env| data.text.clone())
}
