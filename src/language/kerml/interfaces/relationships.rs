// Relationship types for KerML

use super::super::types::VisibilityKind;
use super::annotations::Annotation;
use super::elements::{Element, Feature, Namespace};
use super::references::ElementReference;

/// Relationship extends Element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relationship {
    pub element: Element,
    pub visibility: Option<VisibilityKind>,
    pub elements: Vec<RelationshipElement>,
    // Edge source
    pub source: Option<Element>,
    pub source_ref: Option<ElementReference>,
    pub source_chain: Option<Feature>,
    // Edge targets
    pub target: Option<Element>,
    pub target_ref: Option<ElementReference>,
    pub target_chain: Option<Feature>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipElement {
    Namespace(Box<Namespace>),
    Relationship(Box<Relationship>),
    Feature(Box<Feature>),
    Annotation(Box<Annotation>),
}

/// Non-standard relationship type so that conjugation and specialization have
/// common base type as both are INHERITING relationships
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inheritance {
    pub relationship: Relationship,
}

/// Unioning extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unioning {
    pub relationship: Relationship,
}

/// Differencing extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Differencing {
    pub relationship: Relationship,
}

/// Intersecting extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intersecting {
    pub relationship: Relationship,
}

/// FeatureChaining extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureChaining {
    pub relationship: Relationship,
}

/// Specialization extends Inheritance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specialization {
    pub inheritance: Inheritance,
}

/// Disjoining extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disjoining {
    pub relationship: Relationship,
}

/// FeatureInverting extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureInverting {
    pub relationship: Relationship,
}

/// TypeFeaturing extends Featuring
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFeaturing {
    pub featuring: Featuring,
}

/// FeatureTyping extends Specialization
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureTyping {
    pub specialization: Specialization,
}

/// Subclassification extends Specialization
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subclassification {
    pub specialization: Specialization,
}

/// Subsetting extends Specialization
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subsetting {
    pub specialization: Specialization,
}

/// Conjugation extends Inheritance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjugation {
    pub inheritance: Inheritance,
}

/// Redefinition extends Subsetting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redefinition {
    pub subsetting: Subsetting,
}

/// ReferenceSubsetting extends Subsetting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceSubsetting {
    pub subsetting: Subsetting,
}

/// CrossSubsetting extends Subsetting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossSubsetting {
    pub subsetting: Subsetting,
}

/// Dependency extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub relationship: Relationship,
    pub prefixes: Vec<Annotation>,
    pub client: Vec<ElementReference>,
    pub supplier: Vec<ElementReference>,
}

/// Import extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub relationship: Relationship,
    pub imports_all: bool,
    pub is_recursive: bool,
    pub is_namespace: Option<NamespaceMarker>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceMarker {
    Namespace, // Represents "::*"
}

/// MembershipImport extends Import
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MembershipImport {
    pub import: Import,
}

/// NamespaceImport extends Import
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceImport {
    pub import: Import,
}

/// Membership extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Membership {
    pub relationship: Relationship,
    pub is_alias: bool,
}

/// OwningMembership extends Membership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwningMembership {
    pub membership: Membership,
}

/// FeatureValue extends OwningMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureValue {
    pub owning_membership: OwningMembership,
    pub is_default: bool,
    pub is_initial: bool,
}

/// ElementFilterMembership extends OwningMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementFilterMembership {
    pub owning_membership: OwningMembership,
}

/// Featuring extends Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Featuring {
    pub relationship: Relationship,
}

/// FeatureMembership extends Featuring and OwningMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureMembership {
    pub featuring: Featuring,
    pub owning_membership: OwningMembership,
}

/// EndFeatureMembership extends FeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndFeatureMembership {
    pub feature_membership: FeatureMembership,
}

/// ParameterMembership extends FeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterMembership {
    pub feature_membership: FeatureMembership,
}

/// ResultExpressionMembership extends FeatureMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultExpressionMembership {
    pub feature_membership: FeatureMembership,
}

/// ReturnParameterMembership extends ParameterMembership
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnParameterMembership {
    pub parameter_membership: ParameterMembership,
}
