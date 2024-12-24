#[cfg(test)]
mod test_template {
    use LabelSoft::templategen::init_template;

    #[test]
    fn test_init_template() {
        let template = init_template(
            "test_template".to_string(),
            "tester".to_string(),
            "testingver".to_string(),
        );

        assert!(template.meta.is_some())
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
