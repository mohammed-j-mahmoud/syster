// Union types (sum types) for KerML

use super::super::annotations::Annotation;
use super::super::expressions::{
    CollectExpression, FeatureChainExpression, FeatureReferenceExpression, IndexExpression,
    InvocationExpression, LiteralExpression, MetadataAccessExpression, NullExpression,
    OperatorExpression, SelectExpression,
};
use super::super::relationships::Membership;

/// Inline expression types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineExpression {
    FeatureChain(Box<FeatureChainExpression>),
    Index(Box<IndexExpression>),
    Invocation(Box<InvocationExpression>),
    Literal(Box<LiteralExpression>),
    MetadataAccess(Box<MetadataAccessExpression>),
    Null(Box<NullExpression>),
    Operator(Box<OperatorExpression>),
    Collect(Box<CollectExpression>),
    Select(Box<SelectExpression>),
    FeatureReference(Box<FeatureReferenceExpression>),
}

/// Non-owner types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonOwnerType {
    Membership(Box<Membership>),
    Annotation(Box<Annotation>),
}
