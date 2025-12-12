pub mod ast;
pub mod kerml;
pub mod sysml;

// Re-export for convenience
pub use kerml::KerMLParser;
pub use sysml::SysMLParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_parser_compiles() {
        // Just verify the parser compiles
        let result = SysMLParser::parse(sysml::Rule::file, "");
        assert!(result.is_ok());
    }
}
