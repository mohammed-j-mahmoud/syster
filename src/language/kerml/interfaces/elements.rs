// Element types for KerML

use super::super::types::FeatureDirectionKind;
use super::annotations::AnnotatingElement;
use super::relationships::{
    Differencing, Disjoining, EndFeatureMembership, FeatureChaining, FeatureInverting,
    FeatureMembership, FeatureValue, Import, Inheritance, Intersecting, Membership,
    OwningMembership, Relationship, ResultExpressionMembership, TypeFeaturing, Unioning,
};

/// Base element type in KerML - the root of the element hierarchy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    pub declared_name: Option<String>,
    pub declared_short_name: Option<String>,
}

/// Namespace is an element that contains other elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace {
    pub element: Element,
    pub prefixes: Vec<Box<OwningMembership>>,
    pub children: Vec<NamespaceChild>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceChild {
    Import(Box<Import>),
    Membership(Box<Membership>),
}

/// Type extends Namespace
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    pub namespace: Namespace,
    pub is_sufficient: bool,
    pub is_abstract: Option<AbstractMarker>,
    pub heritage: Vec<Inheritance>,
    pub type_relationships: Vec<TypeOrFeatureRelationship>,
    pub multiplicity: Option<Box<OwningMembership>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractMarker {
    Abstract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeOrFeatureRelationship {
    Type(Box<TypeRelationship>),
    Feature(Box<FeatureRelationship>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeRelationship {
    Unioning(Box<Unioning>),
    Differencing(Box<Differencing>),
    Disjoining(Box<Disjoining>),
    Intersecting(Box<Intersecting>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureRelationship {
    TypeRelationship(Box<TypeRelationship>),
    FeatureChaining(Box<FeatureChaining>),
    FeatureInverting(Box<FeatureInverting>),
    TypeFeaturing(Box<TypeFeaturing>),
}

/// Classifier extends Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classifier {
    pub type_: Type,
}

/// DataType extends Classifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataType {
    pub classifier: Classifier,
}

/// Class extends Classifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class {
    pub classifier: Classifier,
}

/// Structure extends Class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Structure {
    pub class: Class,
}

/// Behavior extends Class
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Behavior {
    pub class: Class,
}

/// Association extends Classifier and Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Association {
    pub classifier: Classifier,
    pub relationship: Relationship,
}

/// AssociationStructure extends Association and Structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssociationStructure {
    pub association: Association,
    pub structure: Structure,
}

/// Metaclass extends Structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metaclass {
    pub structure: Structure,
}

/// SysMLFunction extends Behavior
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysMLFunction {
    pub behavior: Behavior,
    pub result: Option<ResultExpressionMembership>,
}

/// Predicate extends SysMLFunction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Predicate {
    pub function: SysMLFunction,
}

/// Interaction extends Association and Behavior
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interaction {
    pub association: Association,
    pub behavior: Behavior,
}

/// Feature extends Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    pub type_: Type,
    pub is_nonunique: bool,
    pub is_ordered: bool,
    pub direction: Option<FeatureDirectionKind>,
    pub is_composite: Option<CompositeMarker>,
    pub is_derived: Option<DerivedMarker>,
    pub is_end: Option<EndMarker>,
    pub is_portion: Option<PortionMarker>,
    pub is_readonly: Option<ReadonlyMarker>,
    pub value: Option<Box<FeatureValue>>,
    pub write: Option<Box<Membership>>,
    pub crossing_feature: Option<Box<OwningMembership>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositeMarker {
    Composite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedMarker {
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndMarker {
    End,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortionMarker {
    Portion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadonlyMarker {
    Readonly,
}

/// MetadataFeature extends Feature and AnnotatingElement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataFeature {
    pub feature: Feature,
    pub annotating_element: AnnotatingElement,
}

/// Multiplicity extends Feature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Multiplicity {
    pub feature: Feature,
}

/// MultiplicityRange extends Multiplicity
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiplicityRange {
    pub multiplicity: Multiplicity,
    pub range: Option<Box<OwningMembership>>,
}

/// ItemFeature extends Feature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemFeature {
    pub feature: Feature,
}

/// Step extends Feature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub feature: Feature,
}

/// Connector extends Feature and Relationship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connector {
    pub feature: Feature,
    pub relationship: Relationship,
    pub ends: Vec<EndFeatureMembership>,
}

/// ItemFlowEnd extends Feature
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemFlowEnd {
    pub feature: Feature,
}

/// Succession extends Connector
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Succession {
    pub connector: Connector,
}

/// BindingConnector extends Connector
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingConnector {
    pub connector: Connector,
}

/// Expression extends Step
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub step: Step,
    pub result: Option<ResultExpressionMembership>,
}

/// ItemFlow extends Connector and Step
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemFlow {
    pub connector: Connector,
    pub step: Step,
    pub item: Option<Box<FeatureMembership>>,
}

/// SuccessionItemFlow extends ItemFlow and Succession
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuccessionItemFlow {
    pub item_flow: ItemFlow,
    pub succession: Succession,
}

/// BooleanExpression extends Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanExpression {
    pub expression: Expression,
}

/// Invariant extends BooleanExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invariant {
    pub boolean_expression: BooleanExpression,
    pub is_negated: bool,
}

/// Package extends Namespace
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub namespace: Namespace,
}

/// LibraryPackage extends Package
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LibraryPackage {
    pub package: Package,
    pub is_standard: bool,
}
