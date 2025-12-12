use super::enums::{DefinitionKind, DefinitionMember, Element, UsageKind, UsageMember};

#[derive(Debug, Clone, PartialEq)]
pub struct SysMLFile {
    pub namespace: Option<NamespaceDeclaration>,
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDeclaration {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Package {
    pub name: Option<String>,
    pub elements: Vec<Element>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    pub kind: DefinitionKind,
    pub name: Option<String>,
    pub body: Vec<DefinitionMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Usage {
    pub kind: UsageKind,
    pub name: Option<String>,
    pub body: Vec<UsageMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub is_recursive: bool,
}
