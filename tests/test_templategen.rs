mod util;

#[cfg(test)]
pub mod test_template {
    use LabelSoft::{
        data::DataProjectSchema,
        templategen::{PageStyleConfig, TemplateProject, templates::template::{Image, Template, Text}},
        
        // LabelSoft:: ::template::{Image, Template, Text},
    };

    use super::util::{basic_dataproject, basic_page_style, basic_schema, basic_template};


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
        assert_eq!(template_project.template.root.len(), 0);
    }

    #[test]
    fn test_add_display_objects() {
        let init_page_style =
            PageStyleConfig::new("DEFAULT".to_string(), 1280.0, 720.0, [1.0, 1.0, 1.0, 1.0])
                .unwrap();

        let template = basic_template(init_page_style);
        let schema = basic_schema();

        let mut templateproject = TemplateProject::new(template, schema);

        // ==== ADD TEXT
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

        // ==== ADD IMAGE
        let image = Image::new(
            1.0,
            1.0,
            100.0,
            100.0,
            "tests/assets/examplebox.png".to_string(),
        )
        .unwrap();
        let add_image_res = templateproject.add_image(image);
        assert!(add_image_res.is_ok());
        assert_eq!(templateproject.template.root.len(), 2);
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
        )
        .unwrap();
        // only check once it has been added to a project which contains the schema.
        assert!(templateproject.add_text(text_invalid_column_name).is_err());
    }

    #[test]
    fn test_add_image_invalid() {
        let style = basic_page_style(&"DEFAULT".to_string());
        let template = basic_template(style);
        let schema = basic_schema();
        let mut templateproject = TemplateProject::new(template, schema);

        // invalid image path
        let image_invalid_path = Image::new(
            0.0,
            0.0,
            1.0,
            1.0,
            "INVALIDIMAGEPATH".to_string()
        );
        assert!(image_invalid_path.is_err());

        // invalid measurements
        let image_invalid_width = Image::new(
            0.0,
            0.0,
            0.0,
            1.0,
            "tests/assets/examplebox.png".to_string()
        );
        assert!(image_invalid_width.is_err());
        let image_invalid_height = Image::new(
            0.0,
            0.0,
            1.0,
            0.0,
            "tests/assets/examplebox.png".to_string()
        );
        assert!(image_invalid_height.is_err());
    }
}
