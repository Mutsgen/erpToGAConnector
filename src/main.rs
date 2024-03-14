use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};
use druid::text::{ParseFormatter};
use regex::Regex;
use reqwest::Error;


const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0))
        .resizable(false);

    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
    };
    let url = "https://petstore.swagger.io/v2/pet/1"; // Замените на URL вашего API
    let data = fetch_data(url).await?;
    println!("{}", data);

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    Ok(())
}

async fn fetch_data(url: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    Ok(body)
}

fn format_str_date(string: &str) -> String {
    // Убедимся, что строка содержит только цифры и имеет длину не менее 8 символов
    let re = Regex::new(r"[^\d]").unwrap();
    let mut _string = string.to_string();
    if re.replace_all(&_string, "").len() < 8 {
        _string = "01011970".to_string();
    }

    // Извлечем день, месяц и год
    let cleaned_string = re.replace_all(&_string, "");
    let (days, month, year) = (&cleaned_string[0..2], &cleaned_string[2..4], &cleaned_string[4..8]);
    let (days, month, year) = (days.parse::<u32>(), month.parse::<u32>(), year.parse::<u32>());

    // Проверим, что день, месяц и год находятся в допустимых диапазонах
    match (days, month, year) {
        (Ok(d), Ok(m), Ok(y)) if d <= 31 && m <= 12 && y >= 1900 && y <= 2100 => format!("{:02}.{:02}.{}", d, m, y),
        _ => "Invalid date".to_string(),
    }
}

fn build_root_widget() -> impl Widget<HelloState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .with_formatter(ParseFormatter::<String>::with_format_fn(|to_format_str: &String| format_str_date(to_format_str.as_str())))
        .fix_width(TEXT_BOX_WIDTH)
        .lens(HelloState::name);

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)
}

