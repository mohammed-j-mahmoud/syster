// Reference types for KerML

use super::elements::Element;

/// ElementReference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementReference {
    pub parts: Vec<Element>,
}

/// NamespaceReference extends ElementReference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceReference {
    pub element_reference: ElementReference,
}

/// TypeReference extends NamespaceReference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeReference {
    pub namespace_reference: NamespaceReference,
}

/// ClassifierReference extends TypeReference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassifierReference {
    pub type_reference: TypeReference,
}

/// FeatureReference extends TypeReference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureReference {
    pub type_reference: TypeReference,
}

/// MetaclassReference extends ClassifierReference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaclassReference {
    pub classifier_reference: ClassifierReference,
}

/// MembershipReference extends ElementReference
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipReference {
    pub element_reference: ElementReference,
}
