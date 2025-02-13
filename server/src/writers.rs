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
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::BufWriter,
        path::Path,
    };

    use ::image::{DynamicImage, GenericImageView};
    use eframe::epaint::image;
    use printpdf::{
        font, ImageRotation, ImageTransform, ImageXObject, IndirectFontRef, Mm, PdfDocument,
        PdfDocumentReference, PdfLayer, PdfLayerReference, PdfPageReference, Pt,
    };

    use crate::{
        renderables::{ImageRenderable, RenderableBuilder, RenderablePage, TextRenderable},
        templategen::templates::template::{PageStyle, Position, Text},
    };

    use super::{generate_output_directories, Writer};

    #[derive(Debug)]
    pub struct PDFWriter {
        __file_path: String,
        __page_descriptors: Vec<RenderablePage>,
    }

    // ======================
    // SPACE FORMATTING
    // ======================

    /// ### Cursor Instance Struct
    /// Groups cursor tracking variables needed to implement a wrapper for PrintPDF.
    ///
    /// As PrintPDF implements a non-episodic cursor, i.e., positions are relative to the last given position,
    /// an episodic wrapper is made.
    ///
    /// Note that this struct has no implementation. It is utilized by various static methods on the PDFWriter class
    /// like add_text and add_image.
    #[derive(Debug)]
    struct CursorInstance {
        cursor_x: f32,
        cursor_y: f32,
        page_height: f32,
    }

    impl PDFWriter {
        // ### Correct Position
        // Encapsulated logic to convert generic coordinates into the PrintPDF coordinate system.
        fn correct_position(__position: &Position, __page_height: &f32) -> (f32, f32) {
            return (__position.x, -__position.y);
        }

        /// ### Clean Cursor
        /// Moves the cursor back to the initial episode point, closing the episode.
        fn clean_cursor(__cursor_tracker: &mut CursorInstance, __layer: &PdfLayerReference) {
            // find difference needed to move to 0.
            let change_x: Mm = Pt(0.0 - __cursor_tracker.cursor_x).into();
            let change_y: Mm = Pt(0.0 - __cursor_tracker.cursor_y).into();
            __layer.set_text_cursor(change_x, change_y);
        }

        /// ### Move Cursor
        /// Move's the cursor to the provided coordinates.
        ///
        /// ! This function is non-episodic, and coordinates should be given as such.
        /// See CursorInstance for further implementation reasoning.
        ///
        /// ! Note that the coordinates should be generic, not in the inverted PrintPDF method.
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
            let change_x = x - __cursor_tracker.cursor_x;
            // not sure why this is correct
            let change_y = y + __cursor_tracker.cursor_y;

            let mm_x: Mm = Pt(change_x).into();
            let mm_y: Mm = Pt(change_y).into();

            // Actually move the cursor in the layer.
            __layer.set_text_cursor(mm_x, mm_y);
            // Assign new cursor coordinates.
            __cursor_tracker.cursor_x += change_x;
            __cursor_tracker.cursor_y += change_y;
            // println!("Provided: {:?}, Change: {:?}, After: {:?}", __position.x, change_x, __cursor_tracker.cursor_x);
        }

        // ======================
        // CONVERSION AND LOADERS
        // ======================

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

        fn build_font_map(&self, __doc: &PdfDocumentReference) -> HashMap<String, IndirectFontRef> {
            let unique_font_paths: HashSet<String> = self
                .__page_descriptors
                .iter()
                .flat_map(|page| &page.renderables)
                .filter_map(|inner| inner.downcast_ref::<TextRenderable>())
                .map(|text_obj| text_obj.font.clone())
                .collect();
            let mut font_map: HashMap<String, IndirectFontRef> = HashMap::new();
            for font_path in unique_font_paths.iter() {
                // initialize and verify the provided file path.
                let fp = match File::open(&font_path) {
                    Err(e) => {
                        log::error!("Couldn't find file path: {:?}, e:{:?}", font_path, e);
                        continue;
                    }
                    Ok(v) => v,
                };
                // load font from file path and retain indirectpointer. printpdf has its own font wrapper,
                // however we also implement our own.
                let font = match __doc.add_external_font(fp) {
                    Err(e) => {
                        log::error!("Couldn't load font from path: {:?}, e:{:?}.", font_path, e);
                        continue;
                    }
                    Ok(v) => v,
                };
                // save to the map.
                font_map.insert(font_path.to_string(), font);
            }
            return font_map;
        }

        // #[cfg(test)]
        pub fn test_build_font_map(&self) -> HashMap<String, IndirectFontRef> {
            let (doc, _page, _layer) = PdfDocument::new("test", Mm(0.0), Mm(0.0), "test_layer");
            return self.build_font_map(&doc);
        }

        // ======================
        // ADD DOCUMENT OBJECTS
        // ======================

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
            let spacing_for_font_size: f32 = font_size_float;

            // Initialize the tracking cursor instance.
            // The cursor is episodic so we're always working from 0,0.
            let mut cursor_instance = CursorInstance {
                cursor_x: 0.0,
                cursor_y: 0.0,
                page_height: *__page_height_r,
            };

            // set the starting position, aka. the difference from 0.0
            let root_pos = Position {
                x: __text.position.x,
                y: __text.position.y,
            };
            // move to the required position
            Self::move_cursor(&mut cursor_instance, &root_pos, __layer);

            // setup parameters.
            __layer.set_text_rendering_mode(printpdf::TextRenderingMode::FillClip);
            __layer.set_font(__font, font_size_float);

            // write lines in the text object,
            for line in &__text.lines {
                let next_line_pos: Position = Position {
                    x: __text.position.x,
                    y: cursor_instance.cursor_y + spacing_for_font_size,
                };
                Self::move_cursor(&mut cursor_instance, &next_line_pos, __layer);
                __layer.write_text(line, __font);
            }

            // clean the cursor after everything.
            Self::clean_cursor(&mut cursor_instance, __layer);

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

            let translate_x: Mm = Pt(x).into();
            // Using top-left coordinates system.
            // The image is positioned in printpdf from the bottom-left, and the coordinate root is also bottom-left.
            //
            // Move to the top, move to the required position of the image, add on the height of the image.
            let translate_y: Mm = Pt(__page_height_r + y - (__image.height as f32)).into();

            // dpi is 72.0 so that 1px = 1pt.
            // This makes the measurements consistent.
            let transform = ImageTransform {
                translate_x: Some(translate_x),
                translate_y: Some(translate_y),
                rotate: None,
                scale_x: Some(1.0),
                scale_y: Some(1.0),
                dpi: Some(72.0),
            };
            // add the image to the layer.
            // have to clone here for some reason.
            pdf_image.add_to_layer(__layer.clone(), transform);

            Ok(())
        }

        // ======================
        // GENERATION
        // ======================

        fn add_page_elements(
            &self,
            __renderables: &Vec<Box<dyn std::any::Any>>,
            __page_style: &PageStyle,
            __loaded_font_map: &HashMap<String, IndirectFontRef>,
            __layer: &PdfLayerReference,
        ) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
            // tracks the indices (and thus elements) which have been added correctly.
            // this is primarily used for testing.
            let mut added_indices: Vec<usize> = Vec::new();
            // object level iteration.
            for (i, renderableobject) in __renderables.iter().enumerate() {
                // as vec is of any type, cannot use match statement.

                // RENDER TEXT
                if let Some(object) = renderableobject.downcast_ref::<TextRenderable>() {
                    // get the required font pointer for this text.
                    let font = match __loaded_font_map.get(&object.font) {
                        Some(font) => font,
                        None => {
                            log::error!("Couldn't load font from path: {:?}", &object.font);
                            continue;
                        }
                    };

                    let res = match Self::add_text(&__layer, &object, &font, &__page_style.height) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error adding text in PDFWriter: {:?}", e);
                            continue;
                        }
                    };
                // RENDER IMAGES
                } else if let Some(object) = renderableobject.downcast_ref::<ImageRenderable>() {
                    // add image
                    let res = match Self::add_image(&__layer, &object, &__page_style.height) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error adding image in PDFWriter: {:?}", e);
                            continue;
                        }
                    };
                } else {
                    log::error!(
                        "Unknown or Unimplemented Renderable Encountered at index: {:?}",
                        i
                    );
                    continue;
                }
                // add indices to the tracking vector.
                added_indices.push(i);
            }

            return Ok(added_indices);
        }

        pub fn test_add_page_elements(&self) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
            let (doc, _page, _layer) =
                PdfDocument::new("test_output", Mm(0.0), Mm(0.0), "test_layer");

            let loaded_font_map = self.build_font_map(&doc);
            // just run one page for the test. it will be the same on every page anyway.
            let renderablepage = self.__page_descriptors.iter().next().unwrap();
            // handle creation and reference to printpdf page.
            let (current_page, current_layer) = Self::add_page(&doc, &renderablepage.page_style);

            // Iteration method which works through adding each element individually.
            match self.add_page_elements(
                &renderablepage.renderables,
                &renderablepage.page_style,
                &loaded_font_map,
                &current_layer,
            ) {
                Ok(v) => return Ok(v),
                Err(e) => return Err(e),
            };
        }

        // ======================
        // ENTRY POINTS
        // ======================

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

            let top_margin: Mm = Pt(init_page_get.page_style.margins[0]).into();
            let left_margin: Mm = Pt(init_page_get.page_style.margins[3]).into();

            // ============
            // Optimization Collections
            // ============
            let loaded_font_map = self.build_font_map(&doc);

            // ============
            // Generation Loop
            // ============
            // page level iteration.
            for (page_num, renderablepage) in self.__page_descriptors.iter().enumerate() {
                // initialize page with parameters.
                let (current_page, current_layer) =
                    Self::add_page(&doc, &renderablepage.page_style);
                // initialize formatting parameters.
                // move the cursor to the top left (with margins)
                // it initially starts bottom left.
                current_layer.set_text_cursor(left_margin, page_height - top_margin);

                // Iteration method which works through adding each element individually.
                match self.add_page_elements(
                    &renderablepage.renderables,
                    &renderablepage.page_style,
                    &loaded_font_map,
                    &current_layer,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!(
                            "Couldn't add page elements for page: {:?}, e:{:?}",
                            page_num,
                            e
                        );
                        continue;
                    }
                };
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
