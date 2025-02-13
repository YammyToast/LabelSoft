use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::{collections::HashMap, path::Path};

// ======================
// Schema
// ======================
#[derive(Debug, Clone)]
/// ## Data Project Schema
/// Helper object which provides an interface for quickly indexing a dataset.
pub struct DataProjectSchema {
    // String key, points to element's index in record.
    pub cols: HashMap<String, usize>,
    // Number of columns in the DataProject.
    pub num_cols: usize,
}

impl DataProjectSchema {
    /// ## New Data Schema
    /// Initializes a new schema using the columnar headers.
    /// The headers are then used as keys in a hashmap which points to the column's index in the dataset structure.
    ///
    /// Performs sanitization to check that the provided headers are okay; that there are no duplicates, are not empty, etc,.
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
        // Report accrued errors.
        let num_cols = &cols.len();
        if !errors.is_empty() {
            return Err(errors.join(",").into());
        }
        Ok(DataProjectSchema {
            cols: cols,
            num_cols: *num_cols,
        })
    }

    /// Convert a column name into a numerical vector index.
    pub fn get_index(&self, __key: &str) -> Option<&usize> {
        let index = self.cols.get(__key);
        index
    }
}

// ======================
// DataRecord
// ======================
#[derive(Debug, Clone)]
/// ## Data Record
/// Implementation wrapper for parsed records of data.
///
/// Each element is stored as an unprocessed string.
/// The order of the elements is important; the order should match that of the respective data schema.
pub struct DataRecord {
    elements: Vec<String>,
}

impl DataRecord {
    /// Does sanitization on a record of data: checking the number of values is as expected, etc,.
    fn check_against_schema(
        __elements: &Vec<String>,
        __schema: &DataProjectSchema,
    ) -> Option<Box<dyn Error>> {
        if __elements.len() != __schema.num_cols {
            return Some(format!("Number of values does not match columns in schema").into());
        }
        None
    }

    /// ## New Data Record
    /// Create a new record from a string of data.
    ///
    /// ### Parameters
    /// - __record_str: Unprocessed string of data that is to be parsed into the record's elements.
    /// - __schema: Schema to validate/sanitize the record against.
    pub fn new(
        __record_str: &String,
        __schema: &DataProjectSchema,
    ) -> Result<Self, Box<dyn Error>> {
        let elements: Vec<String> = __record_str.split(",").map(|val| val.to_string()).collect();
        match Self::check_against_schema(&elements, __schema) {
            None => {}
            Some(e) => {
                return Err(format!("Could not create data-record with schema: {:?}", e).into())
            }
        }
        Ok(DataRecord { elements: elements })
    }
}

// ======================
// DataRecordIndexed
// ======================

/// ## Data Record Indexed
/// Iteration Helper Object, which is intended to yield both the record and schema in a bundle.
/// Although not strictly necessary, this allows for a record to be indexed in code very neatly.
pub struct DataRecordIndexed {
    pub record: DataRecord,
    schema: DataProjectSchema,
}

/// Implementation for yielding both the record and the respective schema at once.
impl Index<&str> for DataRecordIndexed {
    type Output = String;

    fn index(&self, __key: &str) -> &Self::Output {
        let index = self.schema.get_index(__key).unwrap();
        let element = self.record.elements.index(*index);
        element
    }
}

// ======================
// DataProject
// ======================

#[derive(Debug)]
/// ## Data Project
/// Object that encapsulates the stages of loading and parsing an input data file.
///
/// Implemented such that the process of loading is managed in stages, which mutates
/// variables internally.
/// Additionally, each stage manages errors independently, allowing for easier debugging for the client.
/// 
/// To ensure that the parsed data can be used further in the pipeline, additional methods and helper objects
/// are used to ease the process.
pub struct DataProject {
    pub file_path: String,
    loaded: bool,
    extension: String,
    // === Data
    pub records: Vec<DataRecord>,
    pub schema: DataProjectSchema,
}

impl DataProject {
    /// Method to retrieve just the first line of a file in an efficient manner.
    /// i.e, not reading the whole file just to get the first line.
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

    /// ## Get Schema by File Type
    /// Switch method to define format specific loading logic for different files.
    /// 
    /// ### Parameters
    /// - __path: File path to the data for use in the extension specific algorithm.
    /// - __extension: Extracted extension from the file path. Used to match to specific parsing algorithm.
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
    /// 
    /// ### Parameters
    /// - __fp: File path to data file.
    pub fn new_infer_schema(__fp: &str) -> Option<Self> {
        // check path exists
        let path = Path::new(__fp);
        if path.exists() == false {
            return None;
        }
        // get file extension
        let ext = match path.extension() {
            Some(v) => v.to_str().unwrap(),
            None => {
                log::error!("Could not extract file extension");
                return None;
            }
        };

        // build the schema.
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

    /// ## Read Data CSV
    /// Format specific logic for loading and parsing data from a CSV file.
    /// 
    /// ### Parameters
    /// - __path: Path to the CSV file to load.
    fn read_data_csv(__path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let f = File::open(__path).unwrap();
        let mut reader = BufReader::new(f).lines();
        // skip the first line as we've already read it as the header.
        let _ = reader.next();

        let mut errors: Vec<String> = Vec::new();
        let mut lines: Vec<String> = Vec::new();
        // parse lines by yield.
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

    /// Match function which links to format specific logic.
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

    /// ## Load Data Project
    /// Entry function to load a dataproject which has already been initialized with a schema.
    /// 
    /// Format specific logic is handled inside, so this is a generic method which can be called
    /// assuming the schema prerequisite has been met.
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
                }
                Err(e) => return Err(e),
            }
        }
        self.records = records;
        Ok(())
    }
}

// Iteration logic yields the DataRecordIndexed helper objects from the DataRecords in the Project.
impl IntoIterator for DataProject {
    type Item = DataRecordIndexed;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let indexed_records: Vec<DataRecordIndexed> = self
            .records
            .iter()
            .map(|x| DataRecordIndexed {
                record: x.clone(),
                schema: self.schema.clone(),
            })
            .collect();
        indexed_records.into_iter()
    }
}
