enum Color {
    Black,
    White,
    Red,
    Blue,
}

impl Color {
    fn from_str(value: &str) -> Option<Color> {
        match value {
            "black" => Some(Color::Black),
            "white" => Some(Color::White),
            "red" => Some(Color::Red),
            "blue" => Some(Color::Blue),
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
    let p_background_color = Property::new("background-color", Value::Color(Color::Black));
    let p_color = Property::new("color", Value::Color(Color::Blue));
    let div_background_color = Property::new("background-color", Value::Color(Color::Red));

    let p_rule = StyleRule::new("p", vec![p_background_color, p_color]);
    let div_rule = StyleRule::new("div", vec![div_background_color]);

    let stylesheet = StyleSheet::new(vec![p_rule, div_rule]);
}
