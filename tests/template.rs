#[cfg(test)]
mod test_template {
    use LabelSoft::{
        data::DataProjectSchema,
        templategen::{PageStyleConfig, TemplateProject},
        templates::template::{Template, Text},
    };

    fn basic_page_style(__default_name: &String) -> PageStyleConfig {
        PageStyleConfig::new(__default_name.clone(), 1280.0, 720.0, [1.0, 1.0, 1.0, 1.0]).unwrap()
    }

    fn basic_template(__init_page_style: PageStyleConfig) -> Template {
        Template::new(
            "basic_template".to_string(),
            "tester".to_string(),
            "testver".to_string(),
            __init_page_style,
        )
    }

    fn basic_schema() -> DataProjectSchema {
        let cols: Vec<String> = vec!["order_id", "address", "item_id", "cost"]
            .iter()
            .map(|v| v.to_string())
            .collect();
        let schema = DataProjectSchema::new(cols).unwrap();
        return schema;
    }

    #[test]
    fn test_init_template() {
        let default_name = "DEFAULT".to_string();
        // unwrap as is tested separately.
        let init_page_style = basic_page_style(&default_name);

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
    fn test_create_templateproject() {
        let style = basic_page_style(&"DEFAULT".to_string());
        let template = basic_template(style);

        let schema = basic_schema();
        let template_project = TemplateProject::new(template, schema);
    }

    #[test]
    fn test_add_display_objects() {
        let init_page_style =
            PageStyleConfig::new("DEFAULT".to_string(), 1280.0, 720.0, [1.0, 1.0, 1.0, 1.0])
                .unwrap();

        let template = basic_template(init_page_style);
        let schema = basic_schema();

        let mut templateproject = TemplateProject::new(template, schema);

        let text = Text::new(
            1.0,
            1.0,
            100.0,
            100.0,
            "order_id".to_string(),
            10,
            "test".to_string(),
        )
        .unwrap();

        let add_text_res = templateproject.add_text(text);
        assert!(add_text_res.is_ok());
        assert_eq!(templateproject.template.root.len(), 1);
    }

    #[test]
    fn test_add_text_invalid() {
        let style = basic_page_style(&"DEFAULT".to_string());
        let template = basic_template(style);
        let schema = basic_schema();
        let mut templateproject = TemplateProject::new(template, schema);

        let text_invalid_column_name = Text::new(
            1.0,
            1.0,
            100.0,
            100.0,
            "INVALIDCOLUMNNAME".to_string(),
            10,
            "test".to_string(),
        ).unwrap();

        assert!(templateproject.add_text(text_invalid_column_name).is_err());
    }
}
