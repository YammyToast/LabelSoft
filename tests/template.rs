#[cfg(test)]
mod test_template {
    use LabelSoft::templategen::{init_template, PageStyleConfig};

    #[test]
    fn test_init_template() {
        let default_name = "DEFAULT".to_string();
        // unwrap as is tested separately.
        let init_page_style = PageStyleConfig::new(
            default_name.clone(),
            1280.0,
            720.0,
            [1.0, 1.0, 1.0, 1.0]
        ).unwrap();

        let template = init_template(
            "test_template".to_string(),
            "tester".to_string(),
            "testingver".to_string(),
            init_page_style
        );
        
        assert!(template.meta.is_some());
        assert!(template.page_styles.get(&default_name).is_some());        
    }
    // fn test_create_template() {

    //     let x = create_template();
    //     let y = create_metadata();

    //     let root = create_root();
    //     println!("{:?}", root);

    //     let dobj = create_display_object();
    //     println!("{:?}", dobj);
    // }
}
