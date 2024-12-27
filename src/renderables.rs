use std::any::Any;

use crate::templategen::{TemplateProject};
use crate::templategen::templates::template::Position;


// ======================
// Text Renderable
// ======================

struct TextRenderable {
    position: Position,
    lines: Vec<String>,

}

// ======================
// Renderable Builder
// ======================

struct RenderableBuilder {
    page_object_lists: Vec<Vec<Box<dyn Any>>>
}

impl RenderableBuilder {
    pub fn new_from_templateproject(__templateproject: TemplateProject) -> Self {
        RenderableBuilder { page_object_lists: Vec::new() }
    }
}