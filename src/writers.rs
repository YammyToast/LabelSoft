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

    use ::image::{DynamicImage, GenericImageView};
    use eframe::epaint::image;
    use printpdf::{
        font, ImageRotation, ImageTransform, ImageXObject, IndirectFontRef, Mm, PdfDocument,
        PdfDocumentReference, PdfLayer, PdfLayerReference, PdfPageReference, Pt,
    };

    use crate::{
        renderables::{ImageRenderable, RenderableBuilder, RenderablePage, TextRenderable},
        templategen::templates::template::{PageStyle, Position},
    };

    use super::{generate_output_directories, Writer};

    #[derive(Debug)]
    pub struct PDFWriter {
        __file_path: String,
        __page_descriptors: Vec<RenderablePage>,
    }

    struct CursorInstance {
        cursor_x: f32,
        cursor_y: f32,
        page_height: f32,
    }

    impl PDFWriter {
        fn correct_position(__position: &Position, __page_height: &f32) -> (f32, f32) {
            return (__position.x, -__position.y);
        }

        fn clean_cursor(__cursor_tracker: &mut CursorInstance, __layer: &PdfLayerReference) {
            // find difference needed to move to 0.
            let change_x: Mm = Pt(-__cursor_tracker.cursor_x).into();
            let change_y: Mm = Pt(-__cursor_tracker.cursor_y).into();
            __layer.set_text_cursor(change_x, change_y);
        }

        fn move_cursor(
            __cursor_tracker: &mut CursorInstance,
            __position: &Position,
            __layer: &PdfLayerReference,
        ) {
            // Calculate x and y in the printpdf system. Primarily this is making the y negative.
            let (x, y) = Self::correct_position(__position, &__cursor_tracker.page_height);
            // Now calculate the actual position to move to given the current position.
            // x -> difference positive.
            // y -> difference negative.
            // currently at (10, 10)
            // cursor says (10, -10)
            // ==
            // want to move to (20, 20)
            // which for the cursor is (20, -20)
            // x -> +10
            // y -> -10
            let change_x: Mm = Pt(x - __cursor_tracker.cursor_x).into();
            let change_y: Mm = Pt(y - __cursor_tracker.cursor_y).into();
            // Actually move the cursor in the layer.
            __layer.set_text_cursor(change_x, change_y);
            // Assign new cursor coordinates.
            __cursor_tracker.cursor_x = x;
            __cursor_tracker.cursor_y = y;
        }

        fn dynamicimage2imagexobject(__image: &DynamicImage) -> ImageXObject {
            let (width, height) = __image.dimensions();

            let rgb_data = match __image {
                DynamicImage::ImageRgb8(ref img) => img.as_raw(),
                DynamicImage::ImageRgba8(ref img) => {
                    &img.pixels()
                        .flat_map(|p| p.0[0..3].to_vec()) // Take the first 3 bytes (R, G, B)
                        .collect::<Vec<u8>>()
                }
                _ => panic!("Unsupported image format! Convert the image to RGB or RGBA."),
            };
            let imagexobject = ImageXObject {
                width: printpdf::Px(width as usize),
                height: printpdf::Px(height as usize),
                color_space: printpdf::ColorSpace::Rgb,
                bits_per_component: printpdf::ColorBits::Bit8,
                interpolate: true,
                image_data: rgb_data.clone(),
                smask: None,
                image_filter: None,
                clipping_bbox: None,
            };
            return imagexobject;
        }

        fn add_page(
            __doc: &PdfDocumentReference,
            __style: &PageStyle,
        ) -> (PdfPageReference, PdfLayerReference) {
            let page_width: Mm = Pt(__style.width).into();
            let page_height: Mm = Pt(__style.height).into();

            let page_indices = __doc.add_page(page_width, page_height, "main_layer");
            let page = __doc.get_page(page_indices.0);
            let layer = page.get_layer(page_indices.1);
            return (page, layer);
        }

        fn add_text(
            __layer: &PdfLayerReference,
            __text: &TextRenderable,
            __font: &IndirectFontRef,
            __page_height_r: &f32,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let font_size_float = __text.font_size as f32;
            let spacing_for_font_size: Mm = Pt(-font_size_float).into();

            // let (x, y) = Self::correct_position(&__text.position, __page_height_r);
            // Initialize the tracking cursor instance.
            let mut cursor_instance = CursorInstance {
                cursor_x: 0.0,
                cursor_y: 0.0,
                page_height: *__page_height_r,
            };

            // move to the required position
            Self::move_cursor(&mut cursor_instance, &__text.position, __layer);

            // position text cursor to required position
            // __layer.set_text_cursor(x, y);

            // account for size of the text and the way printpdf handles the y-axis.
            // __layer.set_text_cursor(Mm(0.0), spacing_for_font_size);

            // // setup parameters.
            // __layer.set_line_height(font_size_float);
            // __layer.set_text_rendering_mode(printpdf::TextRenderingMode::FillClip);
            // __layer.set_font(__font, font_size_float);
            // // write lines in the text object,
            // for line in &__text.lines {
            //     __layer.write_text(line, __font);
            //     __layer.set_text_cursor(Mm(0.0), spacing_for_font_size);
            // }

            Ok(())
        }

        fn add_image(
            __layer: &PdfLayerReference,
            __image: &ImageRenderable,
            __page_height_r: &f32,
        ) -> Result<(), Box<dyn std::error::Error>> {
            // convert dynamicimage generic into intermediate ImageXObject type.
            let imagex = Self::dynamicimage2imagexobject(&__image.image_data);
            // create a printpdf image from the ImageXObject intermediate.
            let pdf_image = printpdf::Image::from(imagex);

            let (x, y) = Self::correct_position(&__image.position, __page_height_r);
            let transform = ImageTransform {
                translate_x: Some(Mm(x)),
                translate_y: Some(Mm(y)),
                rotate: None,
                scale_x: None,
                scale_y: None,
                dpi: None,
            };
            // add the image to the layer.
            // have to clone here for some reason.
            pdf_image.add_to_layer(__layer.clone(), transform);

            Ok(())
        }

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

            let (doc, _page, _layer) =
                PdfDocument::new("output", page_width, page_height, "main_layer");
            let font = doc
                .add_external_font(File::open("fonts/ARIAL.TTF").unwrap())
                .unwrap();

            let top_margin: Mm = Pt(init_page_get.page_style.margins[0]).into();
            let left_margin: Mm = Pt(init_page_get.page_style.margins[3]).into();
            // page level iteration.
            for renderablepage in &self.__page_descriptors {
                // initialize page with parameters.
                let (current_page, current_layer) =
                    Self::add_page(&doc, &renderablepage.page_style);
                // initialize formatting parameters.
                // move the cursor to the top left (with margins)
                // it initially starts bottom left.
                current_layer.set_text_cursor(left_margin, page_height - top_margin);

                // object level iteration.
                for renderableobject in &renderablepage.renderables {
                    // as vec is of any type, cannot use match statement.

                    // RENDER TEXT
                    if let Some(object) = renderableobject.downcast_ref::<TextRenderable>() {
                        // add text
                        let res = match Self::add_text(
                            &current_layer,
                            &object,
                            &font,
                            &renderablepage.page_style.height,
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error adding text in PDFWriter: {:?}", e);
                                continue;
                            }
                        };
                    // RENDER IMAGES
                    } else if let Some(object) = renderableobject.downcast_ref::<ImageRenderable>()
                    {
                        // add image
                        let res = match Self::add_image(
                            &current_layer,
                            &object,
                            &renderablepage.page_style.height,
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error adding image in PDFWriter: {:?}", e);
                                continue;
                            }
                        };
                    }
                }
            }

            return Ok(doc);
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
