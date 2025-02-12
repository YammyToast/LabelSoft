mod util;

#[cfg(test)]
mod test_writers {
    use std::path::Path;

    use crate::util::{
        basic_templateproject, diagonal_image_populated_renderablebuilder, font_arial_text,
        font_calibri_text, font_montserrat_text, font_roboto_text,
        font_size_text_populated_renderablebuilder, four_corners_image_populated_renderablebuilder,
        four_fonts_text_populated_renderablebuilder, max_height_image_populated_renderablebuilder,
        max_width_image_populated_renderablebuilder, overlap_text_populated_renderablebuilder,
    };

    use super::util::{basic_populated_renderablebuilder, two_text_populated_renderablebuilder};

    use eframe::egui::output;
    use printpdf::font;
    use LabelSoft::{
        renderables::RenderableBuilder,
        templategen::templates::template::Text,
        writers::{PDFGeneration::PDFWriter, Writer},
    };
    // generic
    const OK_OUTPUT_PATH: &str = "tests/assets/tmp/ok";
    // text
    const TWO_TEXT_OUTPUT_PATH: &str = "tests/assets/tmp/text_two";
    const OVERLAP_TEXT_OUTPUT_PATH: &str = "tests/assets/tmp/text_overlap";
    const TEXT_FONT_SIZE_OUTPUT_PATH: &str = "tests/assets/tmp/text_font_size";
    const TEXT_FOUR_FONTS_OUTPUT_PATH: &str = "tests/assets/tmp/text_four_fonts";
    // image
    const DIAGONAL_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/img_diagonalimg";
    const MAX_HEIGHT_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/img_maxheight";
    const MAX_WIDTH_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/img_maxwidth";
    const FOUR_CORNERS_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/img_fourcorners";

    // ======================
    // FUNC LEVEL
    // ======================

    #[test]
    fn test_font_loader() {
        let init = basic_templateproject();
        let mut templateproject = init.0;
        let data = init.1;

        templateproject.add_text(font_arial_text()).unwrap();
        templateproject.add_text(font_roboto_text()).unwrap();
        templateproject.add_text(font_calibri_text()).unwrap();
        templateproject.add_text(font_montserrat_text()).unwrap();

        let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
        let mut writer = PDFWriter::new("PLACEHOLDER");
        writer.add_builder_pages(builder).unwrap();

        let font_map = writer.test_build_font_map();
        assert_eq!(font_map.len(), 4);
    }

    #[test]
    fn test_font_loader_invalid_path() {
        let init = basic_templateproject();
        let mut templateproject = init.0;
        let data = init.1;

        // setup font with invalid path
        let invalid_text = Text::new(
            0.0,
            14.0,
            100.0,
            100.0,
            "shipping_address".to_string(),
            12,
            "INVALID_PATH".to_string(),
        )
        .unwrap();

        // valid font 1
        templateproject.add_text(font_arial_text()).unwrap();
        // invalid font
        templateproject.add_text(invalid_text).unwrap();
        // valid font 2
        templateproject.add_text(font_calibri_text()).unwrap();

        let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();
        let mut writer = PDFWriter::new("PLACEHOLDER");
        writer.add_builder_pages(builder).unwrap();

        let font_map = writer.test_build_font_map();
        assert_eq!(font_map.len(), 2);
    }

    #[test]
    fn test_element_add_internal() {
        let mut writer = PDFWriter::new("PLACEHOLDER");
        let builder = basic_populated_renderablebuilder();

        let object_count = builder.get_page_object_count(0).unwrap();

        writer.add_builder_pages(builder).unwrap();

        // run the add method and get returned the indices of all elements correctly added.
        let test_elements_indices_res = writer.test_add_page_elements();
        // assert that the procedure ran correct (not necessarily that all elements added correctly).
        assert!(test_elements_indices_res.is_ok());
        let test_elements_indices = test_elements_indices_res.unwrap();
        // check that in = out
        // the elements in the basic renderablebuilder should always work correctly.
        assert_eq!(test_elements_indices.len(), object_count)
    }

    #[test]
    fn test_element_add_invalid_text() {
        let init = basic_templateproject();
        let mut templateproject = init.0;
        let data = init.1;

        let invalid_text = Text::new(
            0.0,
            0.0,
            100.0,
            100.0,
            "INVALID COLUMN NAME".to_string(),
            12,
            "fonts/ARIAL.TTF".to_string(),
        )
        .unwrap();

        // Add the text but don't check whether it returned an error.
        // Ideally this does return an error, however checking it is what we want from this test.
        let _ = templateproject.add_text(invalid_text);

        let mut writer = PDFWriter::new("PLACEHOLDER");
        let builder = RenderableBuilder::new_from_template_and_data(templateproject, data).unwrap();

        // get the first page as any will work.
        let object_count = builder.get_page_object_count(0).unwrap();

        // check that the added objects is still zero.
        // this should occur if the add_text returns an error.
        assert_eq!(object_count, 0);
    }

    // ======================
    // PDF GENERATION
    // ======================

    /// Test that the standard PDF writer processess successfully.
    /// NOTE that this does not encompass the logic error of output not looking as expected.
    /// The output PDF is generated at tests/assets/tmp/ok.pdf for manual inspection.
    #[test]
    fn test_pdf_writer_ok() {
        // setup path
        let fp = Path::new(OK_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        // get basic renderable.
        let builder = basic_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    #[test]
    fn test_pdf_two_text_generation() {
        let fp = Path::new(TWO_TEXT_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        // get two text renderable builder.
        let builder = two_text_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    #[test]
    fn test_pdf_overlap_text_generation() {
        let fp = Path::new(OVERLAP_TEXT_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        let builder = overlap_text_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    #[test]
    fn test_pdf_text_font_size() {
        let fp = Path::new(TEXT_FONT_SIZE_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        let builder = font_size_text_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    #[test]
    fn test_pdf_text_four_fonts() {
        let fp = Path::new(TEXT_FOUR_FONTS_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        let builder = four_fonts_text_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    #[test]
    fn test_pdf_image_diag() {
        let fp = Path::new(DIAGONAL_IMAGE_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        let builder = diagonal_image_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    #[test]
    fn test_pdf_image_height_max() {
        let fp = Path::new(MAX_HEIGHT_IMAGE_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        let builder = max_height_image_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    #[test]
    fn test_pdf_image_width_max() {
        let fp = Path::new(MAX_WIDTH_IMAGE_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        let builder = max_width_image_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    #[test]
    fn test_pdf_image_four_corners() {
        let fp = Path::new(FOUR_CORNERS_IMAGE_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        let builder = four_corners_image_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    // ======================
    // OTHER FORMATS
    // ======================
}
