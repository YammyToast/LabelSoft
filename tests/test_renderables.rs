mod util;

#[cfg(test)]
mod test_renderables {
    use LabelSoft::templategen::templates::template::Text;
    use LabelSoft::templategen::TemplateProject;
    use LabelSoft::{renderables::RenderableBuilder, templategen::templates::template::Template};

    use super::util::{basic_dataproject, basic_page_style, basic_schema, basic_template};

    #[test]
    fn test_renderablebuilder_init() {
        // build the test template
        let style = basic_page_style(&"DEFAULT".to_string());
        let template = basic_template(style);
        // load the test data
        let dataproject = basic_dataproject();
        // combine
        let mut templateproject = TemplateProject::new(template, dataproject.schema.clone());

        let text = Text::new(
            1.0,
            1.0,
            100.0,
            100.0,
            "product_id".to_string(),
            12,
            "test_font".to_string(),
        )
        .unwrap();
        templateproject.add_text(text).unwrap();

        let builder = RenderableBuilder::new_from_template_and_data(templateproject, dataproject);
    }
}
