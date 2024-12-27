use std::any::Any;

use log::error;

use crate::data::{DataProject, DataProjectSchema, DataRecord, DataRecordIndexed};
use crate::templategen::templates::template::display_object::Content;
use crate::templategen::templates::template::{DisplayObject, Position, Text};
use crate::templategen::TemplateProject;

// ======================
// Text Renderable
// ======================

#[derive(Debug)]
struct TextRenderable {
    position: Position,
    lines: Vec<String>,
    max_width: f32,
    max_height: f32,
    font_size: u32,
    font: String,
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
// Renderable Builder
// ======================

#[derive(Debug)]
pub struct RenderableBuilder {
    page_object_lists: Vec<Vec<Box<dyn Any>>>,
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
                    println!("{:?}", renderable);
                }
                Some(Content::Image(v)) => {
                    println!("{:?}", v);
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
        for recordindexed in __data.into_iter() {
            let template_objects = __templateproject.template.root.clone();
            let generated_objects =
                Self::convert_template_objects(template_objects, recordindexed, &schema);
        }
        RenderableBuilder {
            page_object_lists: Vec::new(),
        }
    }
}
