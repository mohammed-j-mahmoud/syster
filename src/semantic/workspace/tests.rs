#![allow(clippy::unwrap_used)]

use super::*;
use crate::language::sysml::populator::REL_SPECIALIZATION;
use crate::parser::{SysMLParser, sysml::Rule};
use from_pest::FromPest;
use pest::Parser;

#[test]
fn test_workspace_creation() {
    let workspace = Workspace::new();
    assert_eq!(workspace.file_count(), 0);
}

#[test]
fn test_add_file() {
    let mut workspace = Workspace::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), file);

    assert_eq!(workspace.file_count(), 1);
    assert!(workspace.contains_file(&path));
}

#[test]
fn test_populate_single_file() {
    let mut workspace = Workspace::new();

    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::model, source).unwrap();
    let file = SysMLFile::from_pest(&mut pairs).unwrap();

    let path = PathBuf::from("vehicle.sysml");
    workspace.add_file(path.clone(), file);

    let result = workspace.populate_file(&path);
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify symbol was added to the shared symbol table
    let symbol = workspace.symbol_table().lookup("Vehicle");
    assert!(symbol.is_some());
    assert_eq!(symbol.unwrap().source_file(), Some("vehicle.sysml"));
}

#[test]
fn test_populate_multiple_files() {
    let mut workspace = Workspace::new();

    // File 1: Base definition
    let source1 = "part def Vehicle;";
    let mut pairs1 = SysMLParser::parse(Rule::model, source1).unwrap();
    let file1 = SysMLFile::from_pest(&mut pairs1).unwrap();

    // File 2: Derived definition
    let source2 = "part def Car :> Vehicle;";
    let mut pairs2 = SysMLParser::parse(Rule::model, source2).unwrap();
    let file2 = SysMLFile::from_pest(&mut pairs2).unwrap();

    workspace.add_file(PathBuf::from("vehicle.sysml"), file1);
    workspace.add_file(PathBuf::from("car.sysml"), file2);

    let result = workspace.populate_all();
    assert!(result.is_ok(), "Failed to populate: {:?}", result.err());

    // Verify both symbols are in the shared symbol table
    let vehicle = workspace.symbol_table().lookup("Vehicle");
    assert!(vehicle.is_some());
    assert_eq!(vehicle.unwrap().source_file(), Some("vehicle.sysml"));

    let car = workspace.symbol_table().lookup("Car");
    assert!(car.is_some());
    assert_eq!(car.unwrap().source_file(), Some("car.sysml"));

    // Verify the specialization relationship was captured
    let specializes = workspace
        .relationship_graph()
        .get_one_to_many(REL_SPECIALIZATION, "Car");
    assert_eq!(specializes, Some(&["Vehicle".to_string()][..]));
}
