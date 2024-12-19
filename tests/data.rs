
#[cfg(test)]
mod test_csv{
    use LabelSoft::data::DataProject;
    const FP_GOOD: &str = "tests/assets/good.csv";


    #[test]
    fn test_dataproject_good() {
        let data_project: Option<DataProject> = DataProject::new_infer_schema(FP_GOOD);
        assert!(!data_project.is_none());
    }
}