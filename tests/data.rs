#[cfg(test)]
mod test_csv {
    use std::{collections::HashMap, hash::Hash, path::Path};

    use LabelSoft::data::{DataProject, DataProjectSchema};
    const FP_GOOD: &str = "tests/assets/good.csv";

    #[test]
    fn test_headers_csv() {
        let fp = Path::new(FP_GOOD);
        let headers = DataProject::read_first_line(fp);
        assert!(headers.is_ok());
        let hd: Vec<String> = headers.unwrap().split(",").map(|f| f.to_string()).collect();
        let schema = DataProjectSchema::new(hd.clone());
        assert!(schema.is_ok());
        let sc = schema.unwrap();
        assert_eq!(sc.num_cols, hd.len());

        // ==== missing/empty headers
        let hd_missing: Vec<String> = String::from(",,,,,")
            .split(",")
            .map(|f| f.to_string())
            .collect();
        let schema_missing_val = DataProjectSchema::new(hd_missing.clone());
        assert!(schema_missing_val.is_err());
        // unwrap as we know this is an error
        let missing_errors = schema_missing_val.err().unwrap();
        let missing_errors_list: Vec<String> = missing_errors.to_string().split(",").map(|f| f.to_string()).collect();
        println!("{:?}", missing_errors_list);
        assert_eq!(missing_errors_list.len(), 6);
        // ==== duplicate column names

        // ==== special characters
        // ==== leading/trailing spaces
    }

    #[test]
    fn test_dataproject_good() {
        let data_project: Option<DataProject> = DataProject::new_infer_schema(FP_GOOD);
        assert!(!data_project.is_none());
        let dp = data_project.unwrap();
        let expected_schema: HashMap<String, usize> = [
            ("order_id", 0),
            ("customer_id", 1),
            ("product_id", 2),
            ("quantity", 3),
            ("unit_price", 4),
            ("order_date", 5),
            ("shipping_address", 6),
            ("payment_method", 7),
            ("order_status", 8),
            ("total_price", 9),
        ]
        .iter()
        .map(|f| (f.0.to_string(), f.1))
        .collect();

        assert_eq!(dp.schema.cols, expected_schema);
    }
}
