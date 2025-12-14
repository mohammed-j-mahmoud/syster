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

/// Represents relationship information that can be attached to definitions and usages
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Relationships {
    /// Specializations (:> or "specializes")
    pub specializes: Vec<String>,
    /// Redefinitions (:>> or "redefines")
    pub redefines: Vec<String>,
    /// Subsetting (:> or "subsets")
    pub subsets: Vec<String>,
    /// Feature typing (: or "typed by")
    pub typed_by: Option<String>,
    /// References (::> or "references")
    pub references: Vec<String>,
    /// Crosses (=> or "crosses")  
    pub crosses: Vec<String>,

    // Domain-specific SysML relationships
    /// Satisfy (satisfy) - satisfaction of requirements
    pub satisfies: Vec<String>,
    /// Perform (perform) - performance relationships
    pub performs: Vec<String>,
    /// Exhibit (exhibit) - exhibition of states
    pub exhibits: Vec<String>,
    /// Include (include) - use case inclusion
    pub includes: Vec<String>,
    /// Assert (assert) - constraint assertion
    pub asserts: Vec<String>,
    /// Verify (verify) - requirement verification
    pub verifies: Vec<String>,
}

impl Relationships {
    /// Create an empty relationships struct (for tests)
    pub fn none() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    pub kind: DefinitionKind,
    pub name: Option<String>,
    pub relationships: Relationships,
    pub body: Vec<DefinitionMember>,
    // Property modifiers
    #[doc(hidden)]
    pub is_abstract: bool,
    #[doc(hidden)]
    pub is_variation: bool,
}

impl Definition {
    pub fn new(
        kind: DefinitionKind,
        name: Option<String>,
        relationships: Relationships,
        body: Vec<DefinitionMember>,
    ) -> Self {
        Self {
            kind,
            name,
            relationships,
            body,
            is_abstract: false,
            is_variation: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Usage {
    pub kind: UsageKind,
    pub name: Option<String>,
    pub relationships: Relationships,
    pub body: Vec<UsageMember>,
    // Property modifiers
    #[doc(hidden)]
    pub is_derived: bool,
    #[doc(hidden)]
    pub is_readonly: bool,
}

impl Usage {
    /// Create a new Usage with default property flags
    pub fn new(
        kind: UsageKind,
        name: Option<String>,
        relationships: Relationships,
        body: Vec<UsageMember>,
    ) -> Self {
        Self {
            kind,
            name,
            relationships,
            body,
            is_derived: false,
            is_readonly: false,
        }
    }
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
