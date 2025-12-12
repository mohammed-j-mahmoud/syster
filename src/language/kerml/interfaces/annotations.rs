// Annotation-related types for KerML

/// Annotation is a relationship that associates an element with another element
/// for the purpose of providing additional information or metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Annotation {
    // TODO: Add annotation properties
}

/// An element that can have annotations about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotatingElement {
    pub about: Vec<Annotation>,
}

/// An annotating element that contains textual content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextualAnnotatingElement {
    pub annotating_element: AnnotatingElement,
    pub body: String,
}

/// A comment is a textual annotation with an optional locale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub textual_annotating_element: TextualAnnotatingElement,
    pub locale: Option<String>,
}

/// Documentation is a special kind of comment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Documentation {
    pub comment: Comment,
}

/// A textual representation with a specified language.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextualRepresentation {
    pub textual_annotating_element: TextualAnnotatingElement,
    pub language: String,
}
