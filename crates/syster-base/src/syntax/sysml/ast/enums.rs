use super::types::{Alias, Comment, Definition, Import, Package, Usage};

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Package(Package),
    Definition(Definition),
    Usage(Usage),
    Comment(Comment),
    Import(Import),
    Alias(Alias),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionKind {
    Part,
    Port,
    Action,
    State,
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
    Allocation,
    Calculation,
    Connection,
    Constraint,
    Enumeration,
    Flow,
    Individual,
    Interface,
    Occurrence,
    Metadata,
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
    Enumeration,
    // Domain-specific usage types
    SatisfyRequirement,
    PerformAction,
    ExhibitState,
    IncludeUseCase,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefinitionMember {
    Comment(Box<Comment>),
    Usage(Box<Usage>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsageMember {
    Comment(Comment),
    Usage(Box<Usage>),
}
