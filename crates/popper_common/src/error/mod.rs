use ariadne::{Color, Source};

pub type ColorConfig = std::collections::HashMap<String, Color>;

pub trait Error {
    fn report(&self,
              color: ColorConfig,
              source: &str,
              file: &str);
}

pub fn generate_color() -> ColorConfig {
    let mut color_map = std::collections::HashMap::new();
    color_map.insert("type".to_string(), Color::Red);
    color_map.insert("variable".to_string(), Color::Blue);
    color_map.insert("function".to_string(), Color::Green);
    color_map.insert("constant".to_string(), Color::Yellow);
    color_map.insert("lambda".to_string(), Color::Magenta);
    color_map.insert("local".to_string(), Color::Cyan);
    color_map.insert("global".to_string(), Color::White);
    color_map.insert("keyword".to_string(), Color::RGB(255, 128, 0)); // orange

    color_map
}

pub fn source_to_string(source: &Source) -> String {
    String::from_iter(source.chars())
}