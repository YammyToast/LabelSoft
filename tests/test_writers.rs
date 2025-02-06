mod util;

#[cfg(test)]
mod test_writers {
    use std::{fmt::write, path::Path};

    use crate::util::{diagonal_image_populated_renderablebuilder, four_corners_image_populated_renderablebuilder, max_height_image_populated_renderablebuilder, max_width_image_populated_renderablebuilder, overlap_text_populated_renderablebuilder};

    use super::util::{basic_populated_renderablebuilder, two_text_populated_renderablebuilder};

    use eframe::egui::output;
    use LabelSoft::writers::{PDFGeneration::PDFWriter, Writer};    
    // generic
    const OK_OUTPUT_PATH: &str = "tests/assets/tmp/ok";
    // text
    const TWO_TEXT_OUTPUT_PATH: &str = "tests/assets/tmp/text_two";
    const OVERLAP_TEXT_OUTPUT_PATH: &str = "tests/assets/tmp/text_overlap";
    // image
    const DIAGONAL_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/img_diagonalimg";
    const MAX_HEIGHT_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/img_maxheight";
    const MAX_WIDTH_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/img_maxwidth";
    const FOUR_CORNERS_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/img_fourcorners";

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