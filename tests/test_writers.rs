mod util;

#[cfg(test)]
mod test_writers {
    use std::{fmt::write, path::Path};

    use crate::util::{diagonal_image_populated_renderablebuilder, overlap_text_populated_renderablebuilder};

    use super::util::{basic_populated_renderablebuilder, two_text_populated_renderablebuilder};

    use eframe::egui::output;
    use LabelSoft::writers::{PDFGeneration::PDFWriter, Writer};    

    const OK_OUTPUT_PATH: &str = "tests/assets/tmp/ok";
    const TWO_TEXT_OUTPUT_PATH: &str = "tests/assets/tmp/twotext";
    const OVERLAP_TEXT_OUTPUT_PATH: &str = "tests/assets/tmp/overlap";
    const DIAGONAL_IMAGE_OUTPUT_PATH: &str = "tests/assets/tmp/diagonalimg";

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
    fn test_pdf_image_overlap() {
        let fp = Path::new(DIAGONAL_IMAGE_OUTPUT_PATH);
        let buf = fp.with_extension("pdf");
        let output_path: &str = buf.to_str().unwrap();

        let mut writer = PDFWriter::new(output_path);
        let builder = diagonal_image_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }

    // ======================
    // OTHER FORMATS
    // ======================



}