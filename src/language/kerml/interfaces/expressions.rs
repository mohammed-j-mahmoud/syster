// Expression types for KerML

use super::elements::Expression;
use super::references::ElementReference;
use super::relationships::Membership;

/// LiteralExpression extends Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralExpression {
    pub expression: Expression,
}

/// LiteralBoolean extends LiteralExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralBoolean {
    pub literal_expression: LiteralExpression,
    pub literal: bool,
}

/// LiteralString extends LiteralExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralString {
    pub literal_expression: LiteralExpression,
    pub literal: String,
}

/// LiteralNumber extends LiteralExpression
#[derive(Debug, Clone)]
pub struct LiteralNumber {
    pub literal_expression: LiteralExpression,
    pub literal: f64,
}

impl PartialEq for LiteralNumber {
    fn eq(&self, other: &Self) -> bool {
        self.literal_expression == other.literal_expression && self.literal == other.literal
    }
}

// Manual Eq implementation since f64 doesn't implement Eq
impl Eq for LiteralNumber {}

/// LiteralInfinity extends LiteralExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralInfinity {
    pub literal_expression: LiteralExpression,
}

/// NullExpression extends Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullExpression {
    pub expression: Expression,
}

/// InvocationExpression extends Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvocationExpression {
    pub expression: Expression,
    pub operands: Option<Vec<Box<Expression>>>,
}

/// OperatorExpression extends InvocationExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorExpression {
    pub invocation_expression: InvocationExpression,
    pub operator: Option<String>,
}

/// IndexExpression extends OperatorExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexExpression {
    pub operator_expression: OperatorExpression,
}

/// FeatureChainExpression extends OperatorExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureChainExpression {
    pub operator_expression: OperatorExpression,
}

/// CollectExpression extends OperatorExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectExpression {
    pub operator_expression: OperatorExpression,
}

/// SelectExpression extends OperatorExpression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectExpression {
    pub operator_expression: OperatorExpression,
}

/// FeatureReferenceExpression extends Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureReferenceExpression {
    pub expression: Expression,
    pub membership: Membership,
}

/// MetadataAccessExpression extends Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataAccessExpression {
    pub expression: Expression,
    pub reference: ElementReference,
}
