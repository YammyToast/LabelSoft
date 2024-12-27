use std::{collections::HashMap, path::Path};

pub mod templates {
    pub mod template {
        include!(concat!(env!("OUT_DIR"), "/template.rs"));
    }
}

use crate::data::DataProjectSchema;

use templates::template::{
    display_object::Content, template::Meta, DisplayObject, Image, PageStyle, Position, Template,
    Text,
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
        if __page_width.le(&0.0) {
            return Err(format!("Page width is less than zero: {:?}", __page_width).into());
        }
        // height
        if __page_height.le(&0.0) {
            return Err(format!("Page height is less than zero: {:?}", __page_height).into());
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
            return Err(format!(
                "Schema does not contain column with name: \'{}\'",
                &__text.data_column
            )
            .into());
        }

        let mut object_wrapper = DisplayObject::default();
        object_wrapper.content = Some(Content::Text(__text));
        self.template.root.push(object_wrapper);
        Ok(())
    }

    pub fn add_image(&mut self, __image: Image) -> Result<(), Box<dyn std::error::Error>> {
        let mut object_wrapper = DisplayObject::default();
        object_wrapper.content = Some(Content::Image(__image));
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
}

// ======================
// Text Display Object
// ======================

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

// ======================
// Image Display Object
// ======================

impl Image {
    pub fn new(
        __x_pos: f32,
        __y_pos: f32,
        __width: f32,
        __height: f32,
        __image_path: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut position = Position::default();
        position.x = __x_pos;
        position.y = __y_pos;
        // check that image path exists.
        let img_path = Path::new(&__image_path);
        if !img_path.exists() {
            return Err(format!(
                "Could not create image, path does not exist: {:?}",
                img_path
            )
            .into());
        }
        // check measurements are valid
        if __width.le(&0.0) {
            return Err(format!("Width of image is less than 0: {:?}", __width).into());
        }
        if __height.le(&0.0) {
            return Err(format!("Height of image is less than 0: {:?}", __height).into());
        }

        Ok(Image {
            position: Some(position),
            width: __width,
            height: __height,
            image_path: __image_path,
        })
    }
}
