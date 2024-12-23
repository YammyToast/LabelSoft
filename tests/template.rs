#[cfg(test)]
mod test_template {
    use LabelSoft::{create_template};

    #[test]
    fn test_create_schematic() {
        let x = create_template("1.0".to_string());
    }
}