#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::core::traits::{AstNode, Named};
use crate::core::visitor::{AstVisitor, Visitable};
use crate::parser::sysml::{Rule, SysMLParser};
use from_pest::FromPest;
use pest::Parser;

#[test]
fn test_package_from_pest() {
    let source = "package MyPackage;";
    let mut pairs = SysMLParser::parse(Rule::package_declaration, source).unwrap();

    let package = Package::from_pest(&mut pairs).unwrap();

    assert_eq!(package.name, Some("MyPackage".to_string()));
    assert_eq!(package.elements.len(), 0);
}

#[test]
fn test_part_definition_from_pest() {
    let source = "part def Vehicle;";
    let mut pairs = SysMLParser::parse(Rule::part_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Part);
    assert_eq!(definition.name, Some("Vehicle".to_string()));
    assert_eq!(definition.body.len(), 0);
}

#[test]
fn test_action_definition_from_pest() {
    let source = "action def Drive;";
    let mut pairs = SysMLParser::parse(Rule::action_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Action);
    assert_eq!(definition.name, Some("Drive".to_string()));
    assert_eq!(definition.body.len(), 0);
}

#[test]
fn test_requirement_definition_from_pest() {
    let source = "requirement def SafetyReq;";
    let mut pairs = SysMLParser::parse(Rule::requirement_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Requirement);
    assert_eq!(definition.name, Some("SafetyReq".to_string()));
    assert_eq!(definition.body.len(), 0);
}

#[test]
fn test_port_definition_from_pest() {
    let source = "port def PowerPort;";
    let mut pairs = SysMLParser::parse(Rule::port_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Port);
    assert_eq!(definition.name, Some("PowerPort".to_string()));
}

#[test]
fn test_item_definition_from_pest() {
    let source = "item def Fuel;";
    let mut pairs = SysMLParser::parse(Rule::item_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Item);
    assert_eq!(definition.name, Some("Fuel".to_string()));
}

#[test]
fn test_attribute_definition_from_pest() {
    let source = "attribute def Mass;";
    let mut pairs = SysMLParser::parse(Rule::attribute_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Attribute);
    assert_eq!(definition.name, Some("Mass".to_string()));
}

#[test]
fn test_part_usage_from_pest() {
    let source = "part engine;";
    let mut pairs = SysMLParser::parse(Rule::part_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Part);
    assert_eq!(usage.name, Some("engine".to_string()));
}

#[test]
fn test_action_usage_from_pest() {
    let source = "action start;";
    let mut pairs = SysMLParser::parse(Rule::action_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Action);
    assert_eq!(usage.name, Some("start".to_string()));
}

#[test]
fn test_port_usage_from_pest() {
    let source = "port power;";
    let mut pairs = SysMLParser::parse(Rule::port_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Port);
    assert_eq!(usage.name, Some("power".to_string()));
}

#[test]
fn test_item_usage_from_pest() {
    let source = "item fuel;";
    let mut pairs = SysMLParser::parse(Rule::item_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Item);
    assert_eq!(usage.name, Some("fuel".to_string()));
}

#[test]
fn test_attribute_usage_from_pest() {
    let source = "attribute weight;";
    let mut pairs = SysMLParser::parse(Rule::attribute_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Attribute);
    assert_eq!(usage.name, Some("weight".to_string()));
}

#[test]
fn test_requirement_usage_from_pest() {
    let source = "requirement safetyReq;";
    let mut pairs = SysMLParser::parse(Rule::requirement_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Requirement);
    assert_eq!(usage.name, Some("safetyReq".to_string()));
}

#[test]
fn test_concern_definition_from_pest() {
    let source = "concern def PerformanceConcern;";
    let mut pairs = SysMLParser::parse(Rule::concern_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Concern);
    assert_eq!(definition.name, Some("PerformanceConcern".to_string()));
}

#[test]
fn test_case_definition_from_pest() {
    let source = "case def TestCase;";
    let mut pairs = SysMLParser::parse(Rule::case_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Case);
    assert_eq!(definition.name, Some("TestCase".to_string()));
}

#[test]
fn test_analysis_case_definition_from_pest() {
    let source = "analysis case def StressAnalysis;";
    let mut pairs = SysMLParser::parse(Rule::analysis_case_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::AnalysisCase);
    assert_eq!(definition.name, Some("StressAnalysis".to_string()));
}

#[test]
fn test_verification_case_definition_from_pest() {
    let source = "verification case def SafetyVerification;";
    let mut pairs = SysMLParser::parse(Rule::verification_case_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::VerificationCase);
    assert_eq!(definition.name, Some("SafetyVerification".to_string()));
}

#[test]
fn test_use_case_definition_from_pest() {
    let source = "use case def UserLogin;";
    let mut pairs = SysMLParser::parse(Rule::use_case_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::UseCase);
    assert_eq!(definition.name, Some("UserLogin".to_string()));
}

#[test]
fn test_view_definition_from_pest() {
    let source = "view def SystemView;";
    let mut pairs = SysMLParser::parse(Rule::view_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::View);
    assert_eq!(definition.name, Some("SystemView".to_string()));
}

#[test]
fn test_viewpoint_definition_from_pest() {
    let source = "viewpoint def ArchitecturalViewpoint;";
    let mut pairs = SysMLParser::parse(Rule::viewpoint_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Viewpoint);
    assert_eq!(definition.name, Some("ArchitecturalViewpoint".to_string()));
}

#[test]
fn test_rendering_definition_from_pest() {
    let source = "rendering def DiagramRendering;";
    let mut pairs = SysMLParser::parse(Rule::rendering_definition, source).unwrap();

    let definition = Definition::from_pest(&mut pairs).unwrap();

    assert_eq!(definition.kind, DefinitionKind::Rendering);
    assert_eq!(definition.name, Some("DiagramRendering".to_string()));
}

#[test]
fn test_concern_usage_from_pest() {
    let source = "concern perfConcern;";
    let mut pairs = SysMLParser::parse(Rule::concern_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Concern);
    assert_eq!(usage.name, Some("perfConcern".to_string()));
}

#[test]
fn test_case_usage_from_pest() {
    let source = "case testCase;";
    let mut pairs = SysMLParser::parse(Rule::case_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::Case);
    assert_eq!(usage.name, Some("testCase".to_string()));
}

#[test]
fn test_view_usage_from_pest() {
    let source = "view systemView;";
    let mut pairs = SysMLParser::parse(Rule::view_usage, source).unwrap();

    let usage = Usage::from_pest(&mut pairs).unwrap();

    assert_eq!(usage.kind, UsageKind::View);
    assert_eq!(usage.name, Some("systemView".to_string()));
}

#[test]
fn test_comment_annotation_from_pest() {
    let source = "comment c1;";
    let mut pairs = SysMLParser::parse(Rule::comment_annotation, source).unwrap();

    let comment = Comment::from_pest(&mut pairs).unwrap();

    assert!(!comment.content.is_empty());
}

#[test]
fn test_import_from_pest() {
    let source = "import SomePackage::*;";
    let mut pairs = SysMLParser::parse(Rule::import, source).unwrap();

    let import = Import::from_pest(&mut pairs).unwrap();

    assert!(!import.path.is_empty());
}

impl AstNode for Package {
    fn node_type(&self) -> &'static str {
        "Package"
    }

    fn has_children(&self) -> bool {
        !self.elements.is_empty()
    }
}

impl Named for Package {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl AstNode for Definition {
    fn node_type(&self) -> &'static str {
        "Definition"
    }
}

impl Named for Definition {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[test]
fn test_ast_node_trait() {
    let pkg = Package {
        name: Some("TestPackage".to_string()),
        elements: vec![],
    };

    assert_eq!(pkg.node_type(), "Package");
    assert_eq!(pkg.name(), Some("TestPackage"));
    assert!(!pkg.has_children());
}

#[test]
fn test_definition_traits() {
    let def = Definition {
        kind: DefinitionKind::Part,
        name: Some("Vehicle".to_string()),
        body: vec![],
    };

    assert_eq!(def.node_type(), "Definition");
    assert_eq!(def.name(), Some("Vehicle"));
}

struct CountingVisitor {
    packages: usize,
    definitions: usize,
}

impl AstVisitor for CountingVisitor {
    fn visit_package(&mut self, _package: &Package) {
        self.packages += 1;
    }

    fn visit_definition(&mut self, _definition: &Definition) {
        self.definitions += 1;
    }
}

#[test]
fn test_visitor_pattern() {
    let file = SysMLFile {
        namespace: None,
        elements: vec![
            Element::Package(Package {
                name: Some("TestPkg".to_string()),
                elements: vec![],
            }),
            Element::Definition(Definition {
                kind: DefinitionKind::Part,
                name: Some("TestDef".to_string()),
                body: vec![],
            }),
        ],
    };

    let mut visitor = CountingVisitor {
        packages: 0,
        definitions: 0,
    };

    file.accept(&mut visitor);

    assert_eq!(visitor.packages, 1);
    assert_eq!(visitor.definitions, 1);
}
