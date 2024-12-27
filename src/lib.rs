// Import modules for testing
pub mod data;
pub mod templategen;
pub mod renderables;

pub mod templates {
    pub mod template {
        include!(concat!(env!("OUT_DIR"), "/template.rs"));
    }
}
