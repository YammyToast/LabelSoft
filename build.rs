use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/template.proto"], &["src/proto"]).unwrap();
    Ok(())
}
