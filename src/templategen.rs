use std::collections::HashMap;

use eframe::emath::Float;

use crate::{
    data::DataProjectSchema,
    templates::template::{
        display_object::Content, template::Meta, DisplayObject, Image, PageStyle, Position,
        Template, Text,
    },
};

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
        // ==== value checks
        // width
        if __page_width.is_sign_negative() {
            return Err(format!(
                "Page Style Width is negative or other invalid: {:?}",
                __page_width
            )
            .into());
        }
        if __page_width.eq(&0.0) {
            return Err("Page Style Width is zero and therefore invalid.".into());
        }
        // height
        if __page_height.is_sign_negative() {
            return Err(format!(
                "Page Style Height is negative or other invalid: {:?}",
                __page_height
            )
            .into());
        }
        if __page_height.eq(&0.0) {
            return Err("Page Style Height is zero and therefore invalid.".into());
        }
        // ==== init
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

pub struct TemplateProject {
    pub template: Template,
    pub schema: DataProjectSchema,
}

impl TemplateProject {
    pub fn new(__template: Template, __schema: DataProjectSchema) -> Self {
        TemplateProject {
            template: __template,
            schema: __schema,
        }
    }
    pub fn add_text(&mut self, __text: Text) -> Result<(), Box<dyn std::error::Error>> {
        // check that data column exists in the schema.
        if !self.schema.cols.contains_key(&__text.data_column) {
            return Err(format!("Schema does not contain column with name: \'{}\'", &__text.data_column).into())
        }
        
        let mut object_wrapper = DisplayObject::default();
        object_wrapper.content = Some(Content::Text(__text));
        self.template.root.push(object_wrapper);
        Ok(())
    }
}   

impl Template {
    pub fn new(
        __display_name: String,
        __author: String,
        __software_version: String,
        __default_page_style: PageStyleConfig,
    ) -> Self {
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

    // pub fn add_text(&mut self, __text_obj: Text) -> Result<(), Box<dyn std::error::Error>> {
    //     let mut object_wrapper = DisplayObject::default();
    //     object_wrapper.content = Some(Content::Text(__text_obj));
    //     self.root.push(object_wrapper);
    //     Ok(())
    // }
}

impl Text {
    pub fn new(
        __x_pos: f32,
        __y_pos: f32,
        __max_width: f32,
        __max_height: f32,
        __data_column_name: String,
        __font_size: u32,
        __font: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut position = Position::default();
        position.x = __x_pos;
        position.y = __y_pos;

        Ok(Text {
            position: Some(position),
            max_width: __max_width,
            max_height: __max_height,
            data_column: __data_column_name,
            font_size: __font_size,
            font: __font,
        })
    }
}
