use from_pest::FromPest;
use pest::Parser;
use std::env;
use std::fs;
use syster::language::sysml::syntax::SysMLFile;
use syster::parser::{SysMLParser, sysml::Rule};

fn main() {
    let path = env::args().nth(1).expect("Usage: parse_test <file>");
    let content = fs::read_to_string(&path).expect("Failed to read file");

    match SysMLParser::parse(Rule::model, &content) {
        Ok(mut pairs) => match SysMLFile::from_pest(&mut pairs) {
            Ok(_) => println!("✓ Successfully parsed: {}", path),
            Err(e) => println!("✗ AST error in {}: {:?}", path, e),
        },
        Err(e) => {
            println!("✗ Parse error in {}:", path);
            println!("{}", e);
        }
    }
}
