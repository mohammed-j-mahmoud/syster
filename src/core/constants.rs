/// Supported file extensions for SysML and KerML files
pub const SUPPORTED_EXTENSIONS: &[&str] = &["sysml", "kerml"];

/// SysML file extension
pub const SYSML_EXT: &str = "sysml";

/// KerML file extension
pub const KERML_EXT: &str = "kerml";

/// Checks if a file extension is supported
pub fn is_supported_extension(ext: &str) -> bool {
    SUPPORTED_EXTENSIONS.contains(&ext)
}
