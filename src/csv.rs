use std::{collections::HashMap, path::Path};


// ======================
// Schema
// ======================
#[derive(Debug)]
struct DataProjectSchema {
    // String key, points to element's index in record.
    cols: HashMap<String, usize>,
    // Number of columns in the DataProject.
    num_cols: usize
}

impl DataProjectSchema {
    pub fn new(__cols: Vec<String>) -> Self {
        let mut cols: HashMap<String, usize> = HashMap::new();
        for col in __cols.iter().enumerate() {
            cols.insert(col.1.to_string(), col.0);
        };
        let num_cols = &cols.len();
        DataProjectSchema {
            cols: cols,
            num_cols: *num_cols 
        }   
    }

}


// ======================
// DataProject
// ======================

#[derive(Debug)]
/// # Data Project
/// 
pub struct DataProject {
    file_path: String,
    loaded: bool,
    // === Data
    raw_string: String,
    pub records: Vec<()>,
}


impl DataProject {
    /// ## New-Without-Schema
    /// Initialize a DataProject without knowing the schema beforehand.
    /// This should be used when the user wants to, for example, create a
    /// new template with a data schema that the program has not yet seen
    /// before.
    /// 
    /// This function initialization a new project, and is representative of
    /// one file that the program is currently handling.
    /// The schema is extracted and saved internally, for when the data is 
    /// later loaded, or for auxiliary introspection.
    /// 
    /// __! This function does not load and parse data !__
    /// 
    /// Passed file path will be verified to exist.
    pub fn new_without_schema(__fp: &str) -> Option<Self> {
        let path = Path::new(__fp);
        if path.exists() == false {
            return None;
        }
        Some(DataProject {
            file_path: __fp.to_string(),
            loaded: false,
            raw_string: String::new(),
            records: Vec::new(),
        })
    }
}
