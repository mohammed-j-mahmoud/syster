// Simple enum types for KerML

/// Visibility modifier for elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisibilityKind {
    Private,
    Protected,
    Public,
}

/// Direction of feature flow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FeatureDirectionKind {
    In,
    InOut,
    Out,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    BitwiseNot,
}

/// Classification test operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassificationTestOperator {
    At,
    HasType,
    IsType,
}

/// Equality comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EqualityOperator {
    NotEqual,
    NotIdentical,
    Equal,
    Identical,
}

/// Import kinds for namespaces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportKind {
    /// ::*
    Members,
    /// ::**
    MembersRecursive,
    /// ::*::**
    AllRecursive,
}

/// Relational comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationalOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}
