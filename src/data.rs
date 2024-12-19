use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::ops::Not;
use std::{collections::HashMap, path::Path};

use log::{debug, error, info, log_enabled, Level, Log};
// ======================
// Schema
// ======================
#[derive(Debug, Clone)]
pub struct DataProjectSchema {
    // String key, points to element's index in record.
    pub cols: HashMap<String, usize>,
    // Number of columns in the DataProject.
    pub num_cols: usize,
}

impl DataProjectSchema {
    pub fn new(__cols: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut cols: HashMap<String, usize> = HashMap::new();
        let mut errors: Vec<String> = Vec::new();
        for col in __cols.iter().enumerate() {
            let key = col.1.trim();
            // check if value is empty
            if key.is_empty() {
                errors.push(format!(
                    "Header with no value in column: {:?} of file",
                    col.0
                ));
                continue;
            }
            // check if column already exists/is a duplicate
            if cols.get(key).is_some() {
                errors.push(format!(
                    "Header with value: \'{:?}\' already exists at column: {:?}",
                    key, col.0
                ));
                continue;
            }

            cols.insert(key.to_string(), col.0);
        }
        let num_cols = &cols.len();
        if !errors.is_empty() {
            return Err(errors.join(",").into());
        }
        Ok(DataProjectSchema {
            cols: cols,
            num_cols: *num_cols,
        })
    }
}

// ======================
// DataRecord
// ======================
#[derive(Debug)]
struct DataRecord {
    elements: Vec<String>,
}

impl DataRecord {
    fn check_against_schema(
        __elements: &Vec<String>,
        __schema: &DataProjectSchema,
    ) -> Option<Box<dyn Error>> {
        if __elements.len() != __schema.num_cols {
            return Some(format!("Number of values does not match columns in schema").into())
        }
        None
    }

    pub fn new(
        __record_str: &String,
        __schema: &DataProjectSchema,
    ) -> Result<Self, Box<dyn Error>> {
        let elements: Vec<String> = __record_str.split(",").map(|val| val.to_string()).collect();
        match Self::check_against_schema(&elements, __schema) {
            None => {}
            Some(e) => {
                return Err(format!(
                    "Could not create data-record with schema: {:?}",
                    e
                )
                .into())
            }
        }
        Ok(DataRecord {
            elements: elements,
        })
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
    extension: String,
    // === Data
    pub records: Vec<()>,
    pub schema: DataProjectSchema,
}

impl DataProject {
    pub fn read_first_line(__fp: &Path) -> Result<String, Box<dyn Error>> {
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

    fn get_schema_by_file_type(
        __path: &Path,
        __extension: &str,
    ) -> Result<DataProjectSchema, Box<dyn Error>> {
        // Define behaviour on a file type basis
        match __extension {
            "csv" | ".csv" => {
                let header_str = match Self::read_first_line(__path) {
                    Err(e) => return Err(format!("Could not read CSV headers: {:?}", e).into()),
                    Ok(v) => v,
                };
                let headers = header_str.split(",").map(|val| val.to_string()).collect();
                let schema = match DataProjectSchema::new(headers) {
                    Err(e) => return Err(format!("Could not infer schema: {:?}", e).into()),
                    Ok(v) => v,
                };
                return Ok(schema);
            }
            _ => {
                return Err(format!(
                    "Behaviour for this file type/extension is not implemented: {:?}",
                    __extension
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
        let ext = match path.extension() {
            Some(v) => v.to_str().unwrap(),
            None => {
                log::error!("Could not extract file extension");
                return None;
            }
        };

        let schema = match Self::get_schema_by_file_type(path, ext) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        };
        Some(DataProject {
            file_path: __fp.to_string(),
            loaded: false,
            extension: ext.to_string(),
            records: Vec::new(),
            schema: schema,
        })
    }

    fn read_data_csv(__path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let f = File::open(__path).unwrap();
        let mut reader = BufReader::new(f).lines();
        // skip the first line as we've already read it as the header.
        let _ = reader.next();

        let mut errors: Vec<String> = Vec::new();
        let mut lines: Vec<String> = Vec::new();
        for line in reader {
            match line {
                Err(e) => {
                    errors.push(format!("Could not read line: {}", e));
                    continue;
                }
                Ok(v) => lines.push(v),
            }
        }
        if errors.len() != 0 {
            return Err(errors.join(",").into());
        }
        Ok(lines)
    }

    fn read_data_by_file_type(
        __path: &str,
        __extension: &str,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        match __extension {
            "csv" | ".csv" => return Self::read_data_csv(&__path),
            _ => {
                return Err(format!(
                    "Behaviour for this file type/extension is not implemented: {:?}",
                    __extension
                )
                .into())
            }
        }
    }

    pub fn load(&mut self) -> Result<(), Box<dyn Error>> {
        if self.loaded == true {
            return Err("Project has already been loaded.".into());
        }
    
        let data = match Self::read_data_by_file_type(&self.file_path, &self.extension) {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        let mut records: Vec<DataRecord> = Vec::new();
        for row in data {
            let record = DataRecord::new(&row, &self.schema);
            match record {
                Ok(val) => {
                    records.push(val);
                },
                Err(e) => return Err(e)
            }
        }
        println!("{:?}", records);

        Ok(())
    }
}
