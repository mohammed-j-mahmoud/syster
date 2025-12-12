use super::types::{Comment, Definition, Import, Package, Usage};

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Package(Package),
    Definition(Definition),
    Usage(Usage),
    Comment(Comment),
    Import(Import),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionKind {
    Part,
    Port,
    Action,
    Item,
    Attribute,
    Requirement,
    Concern,
    Case,
    AnalysisCase,
    VerificationCase,
    UseCase,
    View,
    Viewpoint,
    Rendering,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsageKind {
    Part,
    Port,
    Action,
    Item,
    Attribute,
    Requirement,
    Concern,
    Case,
    View,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionMember {
    Comment(Comment),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsageMember {
    Comment(Comment),
}
