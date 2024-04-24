enum Color {
    Black,
    White,
    Red,
}

impl Color {
    fn from_str(value: &str) -> Option<Color> {
        match value {
            "black" => Some(Color::Black),
            "white" => Some(Color::White),
            "red" => Some(Color::Red),
            _ => None,
        }
    }
}

enum Dimension {
    Pixels(f32),
    Ems(f32),
    Percent(f32),
}

enum Value {
    Color(Color),
    Dimension(Dimension),
    Text(String),
}

struct Property {
    name: String,
    value: Value,
}

impl Property {
    fn new(name: &str, value: Value) -> Property {
        Property { name: name.to_string(), value }
    }
}

struct StyleRule {
    selector: String,
    properties: Vec<Property>,
}

impl StyleRule {
    fn new(selector: &str, properties: Vec<Property>) -> StyleRule {
        StyleRule { selector: selector.to_string(), properties }
    }
}

struct StyleSheet {
    rules: Vec<StyleRule>,
}

impl StyleSheet {
    fn new(rules: Vec<StyleRule>) -> StyleSheet {
        StyleSheet { rules }
    }
}

fn main() {
    let bg_color = Property::new("background-color", Value::Color(Color::White));
    let font_size = Property::new("font-size", Value::Dimension(Dimension::Ems(1.2)));
    let font_family = Property::new("font-family", Value::Text("Arial".to_string()));
    let body_rule = StyleRule::new("body", vec![bg_color, font_size, font_family]);
    let stylesheet = StyleSheet::new(vec![body_rule]);
}
