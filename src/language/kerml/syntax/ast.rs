use super::enums::{ClassifierKind, Element, FeatureDirection, ImportKind};
use super::types::{Annotation, Classifier, Comment, Feature, Import, Package};
use crate::language::kerml::model::types::Documentation;
use crate::parser::kerml::Rule;
use from_pest::{ConversionError, FromPest, Void};

macro_rules! impl_from_pest {
    ($type:ty, $body:expr) => {
        impl<'pest> FromPest<'pest> for $type {
            type Rule = Rule;
            type FatalError = Void;

            fn from_pest(
                pest: &mut pest::iterators::Pairs<'pest, Rule>,
            ) -> std::result::Result<Self, ConversionError<Void>> {
                let body_fn: fn(
                    &mut pest::iterators::Pairs<'pest, Rule>,
                ) -> std::result::Result<$type, ConversionError<Void>> = $body;
                body_fn(pest)
            }
        }
    };
}

impl_from_pest!(Package, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::package {
        return Err(ConversionError::NoMatch);
    }
    let mut name = None;
    for inner in pair.into_inner() {
        if matches!(
            inner.as_rule(),
            Rule::identification | Rule::name | Rule::identifier
        ) {
            name = Some(inner.as_str().to_string());
        }
    }
    Ok(Package {
        name,
        elements: Vec::new(),
    })
});

impl_from_pest!(Comment, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::comment {
        return Err(ConversionError::NoMatch);
    }
    let content = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::textual_body)
        .map(|p| p.as_str().to_string())
        .unwrap_or_default();
    Ok(Comment {
        content,
        about: Vec::new(),
        locale: None,
    })
});

impl_from_pest!(Documentation, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::documentation {
        return Err(ConversionError::NoMatch);
    }
    let content = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::textual_body)
        .map(|p| p.as_str().to_string())
        .unwrap_or_default();
    Ok(Documentation {
        comment: Comment {
            content,
            about: Vec::new(),
            locale: None,
        },
    })
});

impl_from_pest!(Classifier, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    let kind = match pair.as_rule() {
        Rule::type_def => ClassifierKind::Type,
        Rule::classifier => ClassifierKind::Classifier,
        Rule::data_type => ClassifierKind::DataType,
        Rule::class => ClassifierKind::Class,
        Rule::structure => ClassifierKind::Structure,
        Rule::behavior => ClassifierKind::Behavior,
        Rule::function => ClassifierKind::Function,
        Rule::association => ClassifierKind::Association,
        Rule::association_structure => ClassifierKind::AssociationStructure,
        Rule::metaclass => ClassifierKind::Metaclass,
        _ => return Err(ConversionError::NoMatch),
    };
    let mut name = None;
    let mut is_abstract = false;
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::abstract_marker => is_abstract = true,
            Rule::identification | Rule::name | Rule::identifier => {
                name = Some(inner.as_str().to_string());
            }
            _ => {}
        }
    }
    Ok(Classifier {
        kind,
        is_abstract,
        name,
        body: Vec::new(),
    })
});

impl_from_pest!(Feature, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::feature {
        return Err(ConversionError::NoMatch);
    }
    let mut name = None;
    let mut direction = None;
    let mut is_readonly = false;
    let mut is_derived = false;
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::identification | Rule::name | Rule::identifier => {
                name = Some(inner.as_str().to_string());
            }
            Rule::readonly => is_readonly = true,
            Rule::derived => is_derived = true,
            Rule::feature_direction_kind => {
                direction = match inner.as_str() {
                    "in" => Some(FeatureDirection::In),
                    "out" => Some(FeatureDirection::Out),
                    "inout" => Some(FeatureDirection::InOut),
                    _ => None,
                };
            }
            Rule::feature_modifier => {
                for modifier in inner.into_inner() {
                    match modifier.as_rule() {
                        Rule::readonly => is_readonly = true,
                        Rule::derived => is_derived = true,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(Feature {
        name,
        direction,
        is_readonly,
        is_derived,
        body: Vec::new(),
    })
});

impl_from_pest!(Import, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::import {
        return Err(ConversionError::NoMatch);
    }
    let mut path = String::new();
    let mut is_recursive = false;
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::imported_reference | Rule::element_reference => {
                path = inner.as_str().to_string();
            }
            Rule::import_kind => {
                is_recursive = inner.as_str().contains("**");
            }
            _ => {}
        }
    }
    Ok(Import {
        path,
        is_recursive,
        kind: ImportKind::Normal,
    })
});

impl_from_pest!(Element, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    Ok(match pair.as_rule() {
        Rule::package => Element::Package(Package::from_pest(&mut pair.into_inner())?),
        Rule::comment => Element::Comment(Comment::from_pest(&mut pair.into_inner())?),
        Rule::type_def
        | Rule::classifier
        | Rule::data_type
        | Rule::class
        | Rule::structure
        | Rule::behavior
        | Rule::function
        | Rule::association
        | Rule::association_structure
        | Rule::metaclass => Element::Classifier(Classifier::from_pest(&mut pair.into_inner())?),
        Rule::feature | Rule::feature_element => {
            Element::Feature(Feature::from_pest(&mut pair.into_inner())?)
        }
        Rule::annotation => Element::Annotation(Annotation {
            reference: pair.as_str().to_string(),
        }),
        Rule::import => Element::Import(Import::from_pest(&mut pair.into_inner())?),
        _ => return Err(ConversionError::NoMatch),
    })
});
