use LabelSoft::{
    data::{DataProject, DataProjectSchema},
    renderables::RenderableBuilder,
    templategen::{
        templates::template::{self, Image, Position, Template, Text},
        PageStyleConfig, TemplateProject,
    },
};

pub const FP_GOOD: &str = "tests/assets/good.csv";

// ======================
// BUNDLING OBJECTS
// ======================

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

// ======================
// PRIMITIVE RENDERING OBJECTS
// ======================

pub fn basic_template_text() -> Text {
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
    return text;
}

pub fn address_template_text() -> Text {
    let address_text = Text::new(
        1.0,
        12.0,
        200.0,
        100.0,
        "shipping_address".to_string(),
        12,
        "test_font".to_string(),
    )
    .unwrap();
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

pub fn root_image() -> Image {
    let image = Image::new(
        0.0,
        0.0,
        128.0,
        128.0,
        "tests/assets/examplebox.png".to_string(),
    )
    .unwrap();
    return image;
}

pub fn diag_image() -> Image {
    let image = Image::new(
        128.0,
        128.0,
        128.0,
        128.0,
        "tests/assets/examplebox.png".to_string(),
    )
    .unwrap();
    return image;
}

pub fn max_height_image() -> Image {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;
    // get the first/default page style.
    let default_page_style = templateproject
        .template
        .page_styles
        .values()
        .next()
        .unwrap();

    let image = Image::new(
        0.0,
        0.0,
        32.0,
        default_page_style.height,
        "tests/assets/bordercross.png".to_string(),
    )
    .unwrap();
    return image;
}

pub fn max_width_image() -> Image {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;
    // get the first/default page style.
    let default_page_style = templateproject
        .template
        .page_styles
        .values()
        .next()
        .unwrap();

    let image = Image::new(
        0.0,
        0.0,
        default_page_style.width,
        32.0,
        "tests/assets/bordercross.png".to_string(),
    )
    .unwrap();
    return image;
}

pub fn corner_image(__position: Position, __width: f32, __height: f32) -> Image {
    let image = Image::new(
        __position.x,
        __position.y,
        __width,
        __height,
        "tests/assets/examplebox.png".to_string(),
    )
    .unwrap();
    return image;
}

// ======================
// SPECIFIC COMPOUND INSTANCES
// ======================

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

pub fn overlap_text_populated_renderablebuilder() -> RenderableBuilder {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;

    let overlap_text = Text::new(
        1.0,
        1.0,
        200.0,
        100.0,
        "shipping_address".to_string(),
        12,
        "test_font".to_string(),
    )
    .unwrap();

    templateproject.add_text(basic_template_text()).unwrap();
    templateproject.add_text(overlap_text).unwrap();

    let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
    return builder;
}

pub fn diagonal_image_populated_renderablebuilder() -> RenderableBuilder {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;

    templateproject.add_image(root_image()).unwrap();
    templateproject.add_image(diag_image()).unwrap();

    let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
    return builder;
}

pub fn max_height_image_populated_renderablebuilder() -> RenderableBuilder {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;

    templateproject.add_image(max_height_image()).unwrap();

    let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
    return builder;
}

pub fn max_width_image_populated_renderablebuilder() -> RenderableBuilder {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;

    templateproject.add_image(max_width_image()).unwrap();

    let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
    return builder;
}

pub fn four_corners_image_populated_renderablebuilder() -> RenderableBuilder {
    let init = basic_templateproject();
    let mut templateproject = init.0;
    let data = init.1;

    let image_height = 64.0;
    let image_width = 64.0;

    // get the default page style.
    let default_page_style = templateproject
        .template
        .page_styles
        .values()
        .next()
        .unwrap();
    let page_height = default_page_style.height;
    let page_width = default_page_style.width;

    // top-left, just 0,0
    let tl_pos = Position { x: 0.0, y: 0.0 };
    templateproject
        .add_image(corner_image(tl_pos, image_width, image_height))
        .unwrap();
    // top-right, page_w - image_width, 0
    let tr_pos = Position {
        x: page_width - image_width,
        y: 0.0,
    };
    templateproject
        .add_image(corner_image(tr_pos, image_width, image_height))
        .unwrap();

    // bottom-left, 0, page_h - image_h
    let bl_pos = Position {
        x: 0.0,
        y: page_height - image_height,
    };
    templateproject
        .add_image(corner_image(bl_pos, image_width, image_height))
        .unwrap();
    // bottom-right, page_w - image_w, page_h - image_h
    let br_pos = Position {
        x: page_width - image_width,
        y: page_height - image_height,
    };
    templateproject
        .add_image(corner_image(br_pos, image_width, image_height))
        .unwrap();

    let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
    return builder;
}
