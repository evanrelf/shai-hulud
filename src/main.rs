use druid::{Data, Env, Lens, LocalizedString, Widget};

fn main() {
    use druid::{AppLauncher, WindowDesc};

    let main_window =
        WindowDesc::new(build_root_widget()).title(LocalizedString::new("Shai-Hulud"));

    let initial_state = State {
        name: String::from("world"),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .unwrap();
}

#[derive(Clone, Data, Lens)]
struct State {
    name: String,
}

fn build_root_widget() -> impl Widget<State> {
    use druid::widget::{CrossAxisAlignment, Flex, Label, TextBox, WidgetExt};

    let text_box = TextBox::new().with_placeholder("Name").lens(State::name);

    let label = Label::new(|data: &State, _env: &Env| format!("Hello, {}!", data.name));

    Flex::column()
        .with_child(text_box)
        .with_child(label)
        .cross_axis_alignment(CrossAxisAlignment::Fill)
}
