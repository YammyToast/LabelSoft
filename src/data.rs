use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{collections::HashMap, path::Path};

use log::{debug, error, info, log_enabled, Level, Log};
// ======================
// Schema
// ======================
#[derive(Debug)]
pub struct DataProjectSchema {
    // String key, points to element's index in record.
    cols: HashMap<String, usize>,
    // Number of columns in the DataProject.
    num_cols: usize,
}

impl DataProjectSchema {
    pub fn new(__cols: Vec<String>) -> Self {
        let mut cols: HashMap<String, usize> = HashMap::new();
        for col in __cols.iter().enumerate() {
            cols.insert(col.1.to_string(), col.0);
        }
        let num_cols = &cols.len();
        DataProjectSchema {
            cols: cols,
            num_cols: *num_cols,
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
    pub schema: DataProjectSchema,
}

impl DataProject {
    fn read_first_line(__fp: &Path) -> Result<String, Box<dyn Error>> {
        // safe to unwrap as fp has been verified.
        let f = File::open(__fp).unwrap();
        // Iterator does not load until yield, thus reading first line is efficient.
        let mut reader = BufReader::new(f).lines();
        let yield_result = reader.next();

        let first_line = match yield_result {
            Some(v) => match v {
                Ok(v) => v,
                Err(e) => return Err(format!("Could not read data: {:?}", e).into()),
            },
            None => return Err("Could not find data in file, likely empty.".into()),
        };
        return Ok(first_line);
    }

    fn get_schema_by_file_type(__path: &Path) -> Result<DataProjectSchema, Box<dyn Error>> {
        let ext = match __path.extension() {
            Some(v) => v.to_str().unwrap(),
            None => return Err("Could not extract file extension".into()),
        };
        // Define behaviour on a file type basis
        match ext {
            "csv" | ".csv" => {
                let header_str = match Self::read_first_line(__path) {
                    Err(e) => return Err(format!("Could not read CSV headers: {:?}", e).into()),
                    Ok(v) => v,
                };
                let headers = header_str.split(",").map(|val| val.to_string()).collect();
                return Ok(DataProjectSchema::new(headers));
            }
            _ => {
                return Err(format!(
                    "Behaviour for this file type/extension is not implemented: {:?}",
                    ext
                )
                .into())
            }
        }
    }

    /// ## New-Infer-Schema
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
    pub fn new_infer_schema(__fp: &str) -> Option<Self> {
        let path = Path::new(__fp);
        if path.exists() == false {
            return None;
        }
        let schema = match Self::get_schema_by_file_type(path) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        };
        Some(DataProject {
            file_path: __fp.to_string(),
            loaded: false,
            raw_string: String::new(),
            records: Vec::new(),
            schema: schema,
        })
    }
}
