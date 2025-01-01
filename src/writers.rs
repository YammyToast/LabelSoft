use std::{fs, path::Path};

use crate::renderables::RenderableBuilder;

pub trait Writer {
    fn new(__file_path: &str) -> Self;
    fn add_builder_pages(
        &mut self,
        __builder: RenderableBuilder,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn write(&mut self) -> Result<String, Box<dyn std::error::Error>>;
}

pub fn generate_output_directories(__fp: &Path) -> Option<Box<dyn std::error::Error>> {
    return match fs::create_dir_all(__fp) {
        Ok(_) => return None,
        Err(e) => Some(
            format!(
                "Could not generate output directories for path: {:?} | {:?}",
                __fp, e
            )
            .into(),
        ),
    };
}

// ======================
// PDF Output
// ======================
pub mod PDFGeneration {
    use std::{fs::File, io::BufWriter, path::Path};

    use printpdf::{Mm, PdfDocument, PdfDocumentReference, Pt};

    use crate::renderables::{RenderableBuilder, RenderablePage};

    use super::{generate_output_directories, Writer};

    #[derive(Debug)]
    pub struct PDFWriter {
        __file_path: String,
        __page_descriptors: Vec<RenderablePage>,
    }

    impl PDFWriter {
        fn generate_pdf(&self) -> Result<PdfDocumentReference, Box<dyn std::error::Error>> {
            // Printpdf requires that the document be initialized with the parameters for the first page,
            // height, width etc, and then returns pointers to the generated page and layer.
            // There currently does not exist a manner in which a document can be generated without
            // an initial page.
            let init_page_get = match self.__page_descriptors.get(0) {
                None => return Err(
                    "Could not get initialize page (at index zero) to get initial page parameters."
                        .into(),
                ),
                Some(v) => v,
            };
            
            let page_width: Mm = Pt(init_page_get.page_style.width).into();
            let page_height: Mm = Pt(init_page_get.page_style.height).into();

            let (doc, _page, _layer) = PdfDocument::new(
                "output",
                page_width,
                page_height,
                "main_layer",
            );

            let font = doc
                .add_external_font(File::open("fonts/ARIAL.TTF").unwrap())
                .unwrap();

            let top_margin: Mm = Pt(init_page_get.page_style.margins[0]).into();
            let left_margin: Mm = Pt(init_page_get.page_style.margins[3]).into();
            let layer = doc.get_page(_page).get_layer(_layer);
            
            layer.begin_text_section();
            layer.set_text_cursor(left_margin, page_height - top_margin);
            layer.set_text_rendering_mode(printpdf::TextRenderingMode::FillClip);
            layer.set_line_height(64.0);
            layer.set_text_cursor(Mm(0.0), Pt(-64.0).into());
            layer.set_font(&font, 64.0);
            
            layer.write_text("test", &font);
            layer.write_text("test", &font);
            layer.write_text("test", &font);
            layer.write_text("test", &font);
            layer.write_text("test", &font);
            layer.end_text_section();
            
            Ok(doc)
        }
    }

    impl Writer for PDFWriter {
        fn new(__file_path: &str) -> Self {
            PDFWriter {
                __file_path: __file_path.to_string(),
                __page_descriptors: Vec::new(),
            }
        }

        fn add_builder_pages(
            &mut self,
            __builder: RenderableBuilder,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let mut pages: Vec<RenderablePage> = __builder.into_iter().collect();
            self.__page_descriptors.append(&mut pages);
            Ok(())
        }

        fn write(&mut self) -> Result<String, Box<dyn std::error::Error>> {
            let self_output_path = Path::new(&self.__file_path);
            let output_dirs = self_output_path.parent().unwrap();
            match generate_output_directories(&output_dirs) {
                Some(e) => return Err(e),
                None => {}
            }

            let doc = match self.generate_pdf() {
                Ok(v) => v,
                Err(e) => {
                    return Err(format!(
                        "Could not generate PDF document with internal method: {}",
                        e
                    )
                    .into())
                }
            };
            doc.save(&mut BufWriter::new(File::create(self_output_path).unwrap()))
                .unwrap();

            let res = format!("Output written to: {}", self.__file_path);
            return Ok(res);
        }
    }
}
