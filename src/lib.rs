// Import modules for testing
pub mod data;


pub mod templates {
    pub mod template {
        include!(concat!(env!("OUT_DIR"), "/template.rs"));
    }
}

use templates::template;

pub fn create_template(__version: String) -> template::Template {
    let mut template = template::Template::default();
    template.version = __version;
    template
}
