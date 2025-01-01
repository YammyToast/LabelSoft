use crate::renderables::RenderableBuilder;

trait Writer {
    fn new(__file_path: &str) -> Self;
    fn add_builder_pages(&self, __builder: RenderableBuilder) -> Result<(), Box<dyn std::error::Error>>;
    fn write() -> Result<String, Box<dyn std::error::Error>>;
}

// ======================
// PDF Output
// ======================
mod PDFGeneration {
    use std::path::Path;

    use crate::renderables::{RenderableBuilder, RenderablePage};

    use super::Writer;

    struct PDFWriter {
        __file_path: String,
        __page_descriptors: Vec<RenderablePage>
    }

    impl Writer for PDFWriter {
        fn new(__file_path: &str) -> Self {
            PDFWriter {
                __file_path: __file_path.to_string(),
                __page_descriptors: Vec::new(),
            }
        }

        fn add_builder_pages(&self, __builder: RenderableBuilder) -> Result<(), Box<dyn std::error::Error>> {
            
        }

        fn write() -> Result<String, Box<dyn std::error::Error>> {
            todo!()
        }
    }
}
