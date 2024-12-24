use std::collections::HashMap;

use eframe::emath::Float;

use crate::templates::template::{template::Meta, PageStyle, Template};

// ======================
// Attribute Bundlers
// ======================

#[derive(Debug)]
pub struct PageStyleConfig {
    style_name: String,
    page_width: f32,
    page_height: f32,
    margins: [f32; 4],
}

impl PageStyleConfig {
    pub fn new(
        __style_name: String,
        __page_width: f32,
        __page_height: f32,
        __margins: [f32; 4],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // value checks
        if __page_width.is_sign_negative() {
            return Err(format!("Page Style Width is negative or other invalid: {:?}", __page_width).into());
        }

        if __page_height.is_sign_negative() {
            return Err(format!("Page Style Height is negative or other invalid: {:?}", __page_height).into());
        }

        Ok(PageStyleConfig {
            style_name: __style_name,
            page_width: __page_width,
            page_height: __page_height,
            margins: __margins,
        })
    }
}

// ======================
// Template Initialization
// ======================

pub fn init_template(
    __display_name: String,
    __author: String,
    __software_version: String,
    __default_page_style: PageStyleConfig,
) -> Template {
    let mut template = Template::default();

    let mut meta = Meta::default();
    meta.display_name = __display_name;
    meta.author = __author;
    meta.software_version = __software_version;

    let mut default_page_style = PageStyle::default();
    default_page_style.style_name = __default_page_style.style_name;
    default_page_style.height = __default_page_style.page_height;
    default_page_style.width = __default_page_style.page_width;
    // cast array of 4 into vector. protobuffers doesn't have arrays afaik.
    default_page_style.margins = __default_page_style.margins.into();
    let mut page_styles = HashMap::new();
    let default_key: String = default_page_style.style_name.clone();
    page_styles.insert(default_key, default_page_style);

    template.meta = Some(meta);
    template.page_styles = page_styles;
    return template;
}

// pub fn create_template(__meta: Meta, __root = Vec<DisplayObject>)
