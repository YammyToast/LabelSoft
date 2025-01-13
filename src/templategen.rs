use std::{collections::HashMap, path::Path};

/// Import protocol buffer codegen.
/// Should other protoc modules be created, they should be added within the first closure.
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
/// Helper object which holds several attributes related to output pages,
/// i.e., page width, page height, etc,.
pub struct PageStyleConfig {
    style_name: String,
    page_width: f32,
    page_height: f32,
    margins: [f32; 4],
}

impl PageStyleConfig {
    /// ## New Page Style Config
    /// Initialize a new page style config group.
    /// 
    /// This only santizies set parameters, and does not perform any internally mutating logic.
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

/// ## Template Project
/// Factory which performs sanitization between a Template and a DataSchema/Dataset.
/// 
/// This is necessary as a Template can be used by multiple different schemas as long as
/// they share the same columns.
/// 
/// For example, a template may have two Text Objects, which refer to columns "id" and "date".
/// Any dataset which has the columns "id" and "date" can be used with the template.
///
/// This factory performs the checks to ensure compatability between the two.
/// 
/// ## Attributes
/// - template: Template half of the merge.
/// - schema: Dataset/Schema half of the merge. 
pub struct TemplateProject {
    pub template: Template,
    pub schema: DataProjectSchema,
}

impl TemplateProject {
    // ## New Template Project
    // Simple internal assign of the two halves.
    pub fn new(__template: Template, __schema: DataProjectSchema) -> Self {
        TemplateProject {
            template: __template,
            schema: __schema,
        }
    }

    /// ## Add Template Text
    /// Adds a Template Text instance to the merger factory.
    /// 
    /// This can fail, as the reference column, e.g.: "id", might not exist in the target schema.
    /// Thus to ensure this doesn't fail, template Text objects should only be added in instances
    /// wherein the column is known to exist.
    /// 
    /// ### Parameters
    /// - __text: Template Text object to add. This is a protocol buffer generated object.
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


    /// ## Add Template Image
    /// Adds a Template Image instance to the merger factory.
    /// 
    /// As this object uses a static image path, this cannot fail with exception to auxiliary issues.
    /// Note that the image path is not checked at this time, and thus the file does not need to exist
    /// at this point in the runtime.
    pub fn add_image(&mut self, __image: Image) -> Result<(), Box<dyn std::error::Error>> {
        let mut object_wrapper = DisplayObject::default();
        object_wrapper.content = Some(Content::Image(__image));
        self.template.root.push(object_wrapper);
        Ok(())
    }
}


/// Attach additional functionality to the Protocolbuffer generated Template class.
/// (The definition of this struct cannot be found in the static codebase, see proto/template.proto)
impl Template {
    /// ## New Template
    /// Create a new serializable template. 
    /// 
    /// This template is not yet populated with objects.
    /// An empty 'root' container is instantiated, which holds instances of 'DisplayObjects'; wrappers
    /// around all potential template type objects.
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


/// Attach additional functionality to the Protocolbuffer generated Template class.
/// (The definition of this struct cannot be found in the static codebase, see proto/display_objects.proto)
impl Text {
    /// ## New Template Text
    /// 
    /// Instantiate a new Template Text. This is an instance of a 'DisplayObject'.
    /// 
    /// This method primarily set the parameters without any mutation.
    /// The X and Y positions are grouped in a protocolbuffer "Position" object for ease of use / interoperability.
    /// 
    /// ### Parameters
    /// - __x_pos: X position (Cartesian) of the text object.
    /// - __y_pos: Y position (Cartesian) of the text object.
    /// - __max_width: Maximum width that the dynamic text value can occupy.
    /// - __max_height: Maximum height that the dynamic text value can occupy.
    /// - __data_column_name: Column name from which a value should be read from a record, in order to populate this text box.
    /// i.e., "order_id" would fill this text box with the respective "order_id" value from the record.
    /// - __font_size: Font-size of the Text once rendered. Also used in text-wrapping calculation.
    /// - __font: TODO! Pointer to the font to use for the Text. Also used in text-wrapping calculations.
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

/// Attach additional functionality to the Protocolbuffer generated Template class.
/// (The definition of this struct cannot be found in the static codebase, see proto/display_objects.proto)
impl Image {
    /// ## New Template Image
    /// 
    /// Instantiate a new Template Image. This is an instance of a 'DisplayObject'.
    /// 
    /// This method primarily set the parameters without any mutation.
    /// The X and Y positions are grouped in a protocolbuffer "Position" object for ease of use / interoperability.
    /// 
    /// ### Parameters
    /// - __x_pos: X position (Cartesian) of the image object.
    /// - __y_pos: Y position (Cartesian) of the image object.
    /// - __width: Width of the image.
    /// - __height: Height of the image.
    /// - __image_path: File path to an image to use.
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
