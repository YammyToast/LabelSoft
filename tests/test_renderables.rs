mod util;

#[cfg(test)]
mod test_renderables {
    use std::path::Path;

    use image::ImageReader;
    use LabelSoft::renderables::ImageRenderable;
    use LabelSoft::templategen::templates::template::{Image, Text};
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

        let image = Image::new(
            1.0,
            200.0,
            64.0,
            64.0,
            "tests/assets/examplebox.png".to_string(),
        )
        .unwrap();
        templateproject.add_image(image).unwrap();

        let builder = RenderableBuilder::new_from_template_and_data(templateproject, dataproject);
    }

    #[test]
    fn test_image_resize_square() {
        let target_width: f32 = 64.0;
        let target_height: f32 = 64.0;
        let image = Image::new(
            1.0,
            1.0,
            target_width,
            target_height,
            "tests/assets/examplebox.png".to_string(),
        )
        .unwrap();
        let resized_res = ImageRenderable::new_from_image(image);
        assert!(resized_res.is_ok());
        // check that output image size is the same as requested.
        let resized = resized_res.unwrap();
        assert_eq!(target_width, resized.image_data.width() as f32);
        assert_eq!(target_height, resized.image_data.height() as f32);
        // ensure that the original image was not this size.
        let path = Path::new("tests/assets/examplebox.png");
        let reader = ImageReader::open(path).unwrap();
        let dyn_img = reader.decode().unwrap();
        assert_ne!(target_height, dyn_img.height() as f32);
        assert_ne!(target_width, dyn_img.width() as f32);
    }
}
