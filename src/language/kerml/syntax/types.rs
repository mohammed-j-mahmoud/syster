use super::enums::{
    ClassifierKind, ClassifierMember, Element, FeatureDirection, FeatureMember, ImportKind,
};
pub use crate::language::kerml::model::types::Comment;

#[derive(Debug, Clone, PartialEq)]
pub struct KerMLFile {
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
pub struct Classifier {
    pub kind: ClassifierKind,
    pub is_abstract: bool,
    pub name: Option<String>,
    pub body: Vec<ClassifierMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Feature {
    pub name: Option<String>,
    pub direction: Option<FeatureDirection>,
    pub is_readonly: bool,
    pub is_derived: bool,
    pub body: Vec<FeatureMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub path: String,
    pub is_recursive: bool,
    pub kind: ImportKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub reference: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Specialization {
    pub general: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Redefinition {
    pub redefined: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subsetting {
    pub subset: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypingRelationship {
    pub typed: String,
}
