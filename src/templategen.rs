
use crate::templates::template::{Template, template::Meta};


pub fn init_template(__display_name: String, __author: String, __software_version: String) -> Template {
    let mut template = Template::default();

    let mut meta = Meta::default();
    meta.display_name = __display_name;
    meta.author = __author;
    meta.software_version = __software_version;
    template.meta = Some(meta);
    return template    
}


// pub fn create_template(__meta: Meta, __root = Vec<DisplayObject>)