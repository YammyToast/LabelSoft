use std::any::Any;
use std::path::Path;

use image::{DynamicImage, ImageReader};
use log::error;

use crate::data::{DataProject, DataProjectSchema, DataRecord, DataRecordIndexed};
use crate::templategen::templates::template::display_object::Content;
use crate::templategen::templates::template::{DisplayObject, Image, Position, Text};
use crate::templategen::TemplateProject;

// ======================
// Text Renderable
// ======================

#[derive(Debug)]
struct TextRenderable {
    pub position: Position,
    pub lines: Vec<String>,
    pub max_width: f32,
    pub max_height: f32,
    pub font_size: u32,
    pub font: String,
}

impl TextRenderable {
    fn wrap_text(__text: String, __max_width: f32, __glyph_width: f32) -> Vec<String> {
        // Calculate wrapping logic here!!!!

        return vec![__text];
    }

    pub fn new_from_text(
        __text: Text,
        __value: String,
    ) -> Result<TextRenderable, Box<dyn std::error::Error>> {
        // glyph width should be derived from font.
        let wrapped = Self::wrap_text(__value, __text.max_width, 1.0);
        if __text.position.is_none() {
            return Err("Position is not set on text object".into());
        }
        let position = __text.position.unwrap();
        // handle height overflow checking here.
        Ok(TextRenderable {
            position: position,
            lines: wrapped,
            max_width: __text.max_width,
            max_height: __text.max_height,
            font_size: __text.font_size,
            font: __text.font,
        })
    }
}

// ======================
// Image Renderable
// ======================

pub struct ImageRenderable {
    pub position: Position,
    pub image_path: String,
    pub image_data: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl ImageRenderable {
    fn handle_image_load(__img_path: &Path) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let ext = match __img_path.extension() {
            Some(ext) => ext,
            None => {
                return Err(format!("Could not get image extension for: {:?}", __img_path).into())
            }
        };
        let data = match ext.to_str().unwrap() {
            "png" | "jpg" | "jpeg" => {
                let reader = ImageReader::open(__img_path).unwrap();
                let decode_result = reader.decode();
                let dyn_image = match decode_result {
                    Ok(v) => v,
                    Err(e) => return Err(e.into())
                };
                dyn_image
            },
            _ => {
                return Err(format!("ImageRenderable is not implemented for extension: {:?}", ext).into())
            }
        };
        return Ok(data)
    }

    pub fn new_from_image(__image: Image) -> Result<Self, Box<dyn std::error::Error>> {
        let img_path = Path::new(&__image.image_path);
        // this should never throw unless this module is used in the
        // wrong order.
        if !img_path.exists() {
            return Err(format!("Image path doesn't exist: {:?}", img_path).into());
        }
        let raw_img = match Self::handle_image_load(img_path) {
            Err(e) => return Err(format!("Couldn't load image data: {:?}", e).into()),
            Ok(data) => data
        };
        
        // resize the image per the user's input.
        let resize_width: u32 = __image.width.floor() as u32;
        let resize_height: u32 = __image.height.floor() as u32;
        let resized = raw_img.resize(resize_width, resize_height, image::imageops::FilterType::Lanczos3);

        let position = __image.position.unwrap();
        Ok(ImageRenderable {
            position: position,
            image_path: __image.image_path,
            image_data: resized,
            width: resize_width,
            height: resize_height,
        })
    }
}

// ======================
// Renderable Builder
// ======================

#[derive(Debug)]
pub struct RenderableBuilder {
    page_grouped_object_lists: Vec<Vec<Box<dyn Any>>>,
}

impl RenderableBuilder {
    fn convert_template_objects(
        __objects: Vec<DisplayObject>,
        __record: DataRecordIndexed,
        __schema: &DataProjectSchema,
    ) -> Vec<Box<dyn Any>> {
        let mut out: Vec<Box<dyn Any>> = Vec::new();
        for object in __objects {
            match object.content {
                Some(Content::Text(v)) => {
                    // get the column, which narrows down to the element/value for this text object.
                    let text_value = &__record[&v.data_column];
                    let renderable = TextRenderable::new_from_text(v, text_value.to_string());
                    match renderable {
                        Err(e) => {
                            error!("Couldn't convert Text into TextRenderable: {:?}", e);
                            continue;
                        }
                        Ok(v) => out.push(Box::new(v)),
                    }
                }
                Some(Content::Image(v)) => {
                    let renderable = ImageRenderable::new_from_image(v);
                    match renderable {
                        Err(e) => {
                            error!("Couldn't convert Image into Image Renderable: {:?}", e);
                            continue;
                        }
                        Ok(v) => out.push(Box::new(v)),
                    }
                }
                None => {
                    error!("Could not convert object: {:?} into renderable", object);
                    continue;
                }
            }
        }
        return out;
    }

    pub fn new_from_template_and_data(
        __templateproject: TemplateProject,
        __data: DataProject,
    ) -> Self {
        let schema = __data.schema.clone();
        let mut page_grouped_object_lists: Vec<Vec<Box<dyn Any>>> = Vec::new();
        // iterate over each record. generated objects are grouped into separate arrays
        // which identify individual pages. one record = one page.
        for recordindexed in __data.into_iter() {
            let template_objects = __templateproject.template.root.clone();
            let generated_objects =
                Self::convert_template_objects(template_objects, recordindexed, &schema);
            page_grouped_object_lists.push(generated_objects);
        }
        RenderableBuilder {
            page_grouped_object_lists: page_grouped_object_lists,
        }
    }
}
