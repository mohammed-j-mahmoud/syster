#[path = "parser/kerml.rs"]
pub mod kerml;
#[path = "parser/sysml.rs"]
pub mod sysml;

// Re-export for convenience
pub use kerml::KerMLParser;
pub use sysml::SysMLParser;
