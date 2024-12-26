#[cfg(test)]
mod test_template {
    use LabelSoft::{
        templategen::PageStyleConfig,
        templates::template::{Template, Text},
    };

    #[test]
    fn test_init_template() {
        let default_name = "DEFAULT".to_string();
        // unwrap as is tested separately.
        let init_page_style =
            PageStyleConfig::new(default_name.clone(), 1280.0, 720.0, [1.0, 1.0, 1.0, 1.0])
                .unwrap();

        let template = Template::new(
            "test_template".to_string(),
            "tester".to_string(),
            "testingver".to_string(),
            init_page_style,
        );

        assert!(template.meta.is_some());
        assert!(template.page_styles.get(&default_name).is_some());
    }

    #[test]
    fn test_page_style_config() {
        // test standard input
        let config_ok = PageStyleConfig::new("Ok".to_string(), 1280.0, 720.0, [1.0, 1.0, 1.0, 1.0]);
        assert!(config_ok.is_ok());
        // test negative values independently
        let config_width_negative =
            PageStyleConfig::new("wn".to_string(), -1.0, 1.0, [0.0, 0.0, 0.0, 0.0]);
        assert!(config_width_negative.is_err());

        let config_height_negative =
            PageStyleConfig::new("hn".to_string(), 1.0, -1.0, [0.0, 0.0, 0.0, 0.0]);
        assert!(config_height_negative.is_err());
        // test zero values independently
        let config_width_zero =
            PageStyleConfig::new("hn".to_string(), 0.0, -1.0, [0.0, 0.0, 0.0, 0.0]);
        assert!(config_width_zero.is_err());
        let config_height_zero =
            PageStyleConfig::new("hn".to_string(), -1.0, 0.0, [0.0, 0.0, 0.0, 0.0]);
        assert!(config_height_zero.is_err());
    }

    #[test]
    fn test_add_display_objects() {
        let init_page_style =
            PageStyleConfig::new("DEFAULT".to_string(), 1280.0, 720.0, [1.0, 1.0, 1.0, 1.0])
                .unwrap();

        let template = Template::new(
            "test_template".to_string(),
            "tester".to_string(),
            "testingver".to_string(),
            init_page_style,
        );

        let text = Text::new(
            1.0,
            1.0,
            100.0,
            100.0,
            "test".to_string(),
            10,
            "test".to_string(),
        );
    }
}
