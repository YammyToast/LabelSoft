mod util;

#[cfg(test)]
mod test_writers {
    use std::{fmt::write, path::Path};

    use super::util::{basic_populated_renderablebuilder};

    use eframe::egui::output;
    use LabelSoft::writers::{PDFGeneration::PDFWriter, Writer};    

    const OK_OUTPUT_PATH: &str = "tests/assets/tmp/ok";

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
        let builder = basic_populated_renderablebuilder();
        writer.add_builder_pages(builder).unwrap();

        let write_res = writer.write();
        assert!(write_res.is_ok());
    }
}