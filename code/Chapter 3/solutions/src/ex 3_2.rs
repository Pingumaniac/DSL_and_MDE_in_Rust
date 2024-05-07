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

struct Property {
    name: String,
    value: Color,
}

impl Property {
    fn new(name: &str, value: &str) -> Result<Property, String> {
        Color::from_str(value)
            .map(|color| Property { name: name.to_string(), value: color })
            .ok_or_else(|| format!("Invalid color value: {}", value))
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
