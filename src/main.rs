use druid::{
    lens,
    lens::Map,
    text::{Formatter, Selection, Validation, ValidationError},
    widget::{
        CrossAxisAlignment, Flex, Label, Painter, SizedBox, TextBox, ValueTextBox, WidgetExt,
    },
    AppLauncher, Color, Data, Env, Lens, LocalizedString, RenderContext, Widget, WindowDesc,
};
use palette::{Hsl, Oklch, Srgb};

struct Arrakis {
    foreground: Oklch,
    background: Oklch,
    white: Oklch,
    black: Oklch,
    blue: Oklch,
    cyan: Oklch,
    green: Oklch,
    yellow: Oklch,
    red: Oklch,
    magenta: Oklch,
    bright_white: Oklch,
    bright_black: Oklch,
    bright_blue: Oklch,
    bright_cyan: Oklch,
    bright_green: Oklch,
    bright_yellow: Oklch,
    bright_red: Oklch,
    bright_magenta: Oklch,
}

fn main() {
    let main_window =
        WindowDesc::new(build_root_widget()).title(LocalizedString::new("Shai-Hulud"));

    let initial_state = State {
        name: String::from("world"),
        color: Srgb::default(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .unwrap();
}

#[derive(Clone, Data, Lens)]
struct State {
    name: String,

    #[data(same_fn = "PartialEq::eq")]
    color: Srgb,
}

fn build_root_widget() -> impl Widget<State> {
    let name_input = TextBox::new().with_placeholder("Name").lens(State::name);
    let name_label = Label::new(|data: &State, _env: &Env| format!("Hello, {}!", data.name));

    let color_input = srgb_input().lens({
        let get = |s: &State| s.color.into_components();
        let put = |s: &mut State, cs| s.color = Srgb::from_components(cs);
        Map::new(get, put)
    });
    let color_label = Label::new(|data: &State, _env: &Env| {
        format!(
            "rgb({}, {}, {})",
            data.color.red, data.color.green, data.color.blue
        )
    });
    let color_swatch = SizedBox::new(Painter::new(|ctx, data: &State, _env: &Env| {
        let bounds = ctx.size().to_rect();
        let color = Color::rgb(
            f64::from(data.color.red),
            f64::from(data.color.green),
            f64::from(data.color.blue),
        );
        ctx.fill(bounds, &color);
    }))
    .expand();

    Flex::column()
        .with_child(name_input)
        .with_child(name_label)
        .with_child(color_input)
        .with_child(color_label)
        .with_child(color_swatch)
        .cross_axis_alignment(CrossAxisAlignment::Fill)
}

fn srgb_input() -> impl Widget<(f32, f32, f32)> {
    let x = ValueTextBox::new(TextBox::new(), F32Formatter).lens(lens!((f32, f32, f32), 0));
    let y = ValueTextBox::new(TextBox::new(), F32Formatter).lens(lens!((f32, f32, f32), 1));
    let z = ValueTextBox::new(TextBox::new(), F32Formatter).lens(lens!((f32, f32, f32), 2));
    Flex::row().with_child(x).with_child(y).with_child(z)
}

struct F32Formatter;

impl Formatter<f32> for F32Formatter {
    fn format(&self, value: &f32) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, _selection: &Selection) -> Validation {
        let regex = regex!(r"\d*(:?\.\d*)?");
        match input.parse::<f32>() {
            Ok(_) => Validation::success(),
            Err(_) if regex.is_match(input) => Validation::success(),
            Err(e) => Validation::failure(e),
        }
    }

    fn value(&self, input: &str) -> Result<f32, ValidationError> {
        input.parse::<f32>().map_err(ValidationError::new)
    }
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
pub(crate) use regex;
