use LabelSoft::{data::{DataProject, DataProjectSchema}, templategen::{templates::template::Template, PageStyleConfig}};

pub const FP_GOOD: &str = "tests/assets/good.csv";



pub fn basic_page_style(__default_name: &String) -> PageStyleConfig {
    PageStyleConfig::new(__default_name.clone(), 1280.0, 720.0, [1.0, 1.0, 1.0, 1.0]).unwrap()
}

pub fn basic_template(__init_page_style: PageStyleConfig) -> Template {
    Template::new(
        "basic_template".to_string(),
        "tester".to_string(),
        "testver".to_string(),
        __init_page_style,
    )
}

pub fn basic_schema() -> DataProjectSchema {
    let cols: Vec<String> = vec!["order_id", "address", "item_id", "cost"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    let schema = DataProjectSchema::new(cols).unwrap();
    return schema;
}

pub fn basic_dataproject() -> DataProject {
    let mut project = DataProject::new_infer_schema(FP_GOOD).unwrap();
    project.load().unwrap();
    return project;
}