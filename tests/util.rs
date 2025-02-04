use LabelSoft::{data::{DataProject, DataProjectSchema}, renderables::RenderableBuilder, templategen::{templates::template::{Image, Template, Text}, PageStyleConfig, TemplateProject}};

pub const FP_GOOD: &str = "tests/assets/good.csv";



pub fn basic_page_style(__default_name: &String) -> PageStyleConfig {
    PageStyleConfig::new(__default_name.clone(), 1280.0, 720.0, [1.0, 1.0, 1.0, 1.0]).unwrap()
}

pub fn basic_template(__init_page_style: PageStyleConfig) -> Template {
    Template::new(
        "basic_template".to_string(),
        "tester".to_string(),
        "testver".to_string(),
        __init_page_style,
    )
}

pub fn basic_schema() -> DataProjectSchema {
    let cols: Vec<String> = vec!["order_id", "address", "item_id", "cost"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    let schema = DataProjectSchema::new(cols).unwrap();
    return schema;
}

pub fn basic_dataproject() -> DataProject {
    let mut project = DataProject::new_infer_schema(FP_GOOD).unwrap();
    project.load().unwrap();
    return project;
}

pub fn basic_templateproject() -> (TemplateProject, DataProject) {
    let style = basic_page_style(&"DEFAULT".to_string());
    let template = basic_template(style);
    let data = basic_dataproject();
    let templateproject = TemplateProject::new(template, data.schema.clone());
    return (templateproject, data);
}

pub fn basic_template_text() -> Text {
    let text = Text::new(
        1.0,
        1.0,
        100.0,
        100.0,
        "product_id".to_string(),
        12,
        "test_font".to_string(),
    ).unwrap();
    return text;
}

pub fn address_template_text() -> Text {
    let address_text = Text::new(
        1.0,
        0.0,
        200.0,
        100.0,
        "shipping_address".to_string(),
        12,
        "test_font".to_string()
    ).unwrap();
    return address_text;
}


pub fn basic_template_image() -> Image {
    let image = Image::new(
        1.0,
        200.0,
        64.0,
        64.0,
        "tests/assets/examplebox.png".to_string(),
    )
    .unwrap();
    return image;
}

pub fn basic_populated_renderablebuilder() -> RenderableBuilder {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;

    templateproject.add_text(basic_template_text()).unwrap();
    templateproject.add_image(basic_template_image()).unwrap();

    let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
    return builder;
}

pub fn two_text_populated_renderablebuilder() -> RenderableBuilder {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;

    templateproject.add_text(basic_template_text()).unwrap();
    templateproject.add_text(address_template_text()).unwrap();

    let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
    return builder;
}