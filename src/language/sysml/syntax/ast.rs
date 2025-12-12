use super::{
    enums::{DefinitionKind, Element, UsageKind},
    types::{Comment, Definition, Import, Package, SysMLFile, Usage},
};
use crate::parser::sysml::Rule;
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
    if pair.as_rule() != Rule::package_declaration {
        return Err(ConversionError::NoMatch);
    }
    let name = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::identification)
        .map(|id| id.as_str().to_string());
    Ok(Package {
        name,
        elements: vec![],
    })
});

impl_from_pest!(Definition, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    let kind = match pair.as_rule() {
        Rule::part_definition => DefinitionKind::Part,
        Rule::action_definition => DefinitionKind::Action,
        Rule::requirement_definition => DefinitionKind::Requirement,
        Rule::port_definition => DefinitionKind::Port,
        Rule::item_definition => DefinitionKind::Item,
        Rule::attribute_definition => DefinitionKind::Attribute,
        Rule::concern_definition => DefinitionKind::Concern,
        Rule::case_definition => DefinitionKind::Case,
        Rule::analysis_case_definition => DefinitionKind::AnalysisCase,
        Rule::verification_case_definition => DefinitionKind::VerificationCase,
        Rule::use_case_definition => DefinitionKind::UseCase,
        Rule::view_definition => DefinitionKind::View,
        Rule::viewpoint_definition => DefinitionKind::Viewpoint,
        Rule::rendering_definition => DefinitionKind::Rendering,
        _ => return Err(ConversionError::NoMatch),
    };

    // Recursively find the identification or identifier
    fn find_name(pairs: pest::iterators::Pairs<Rule>) -> Option<String> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier | Rule::identification => {
                    return Some(pair.as_str().to_string());
                }
                _ => {
                    if let Some(name) = find_name(pair.into_inner()) {
                        return Some(name);
                    }
                }
            }
        }
        None
    }

    let name = find_name(pair.into_inner());

    Ok(Definition {
        kind,
        name,
        body: vec![],
    })
});
impl_from_pest!(Usage, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    let kind = match pair.as_rule() {
        Rule::part_usage => UsageKind::Part,
        Rule::action_usage => UsageKind::Action,
        Rule::requirement_usage => UsageKind::Requirement,
        Rule::port_usage => UsageKind::Port,
        Rule::item_usage => UsageKind::Item,
        Rule::attribute_usage => UsageKind::Attribute,
        Rule::concern_usage => UsageKind::Concern,
        Rule::case_usage => UsageKind::Case,
        Rule::view_usage => UsageKind::View,
        _ => return Err(ConversionError::NoMatch),
    };

    // Recursively find the identification or identifier
    fn find_name(pairs: pest::iterators::Pairs<Rule>) -> Option<String> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::identifier | Rule::identification => {
                    return Some(pair.as_str().to_string());
                }
                _ => {
                    if let Some(name) = find_name(pair.into_inner()) {
                        return Some(name);
                    }
                }
            }
        }
        None
    }

    let name = find_name(pair.into_inner());

    Ok(Usage {
        kind,
        name,
        body: vec![],
    })
});
impl_from_pest!(Comment, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::comment_annotation {
        return Err(ConversionError::NoMatch);
    }
    let content = pair.as_str().to_string();
    Ok(Comment { content })
});

impl_from_pest!(Import, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    if pair.as_rule() != Rule::import {
        return Err(ConversionError::NoMatch);
    }
    let path = pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::imported_reference || p.as_rule() == Rule::identification)
        .map(|p| p.as_str().to_string())
        .unwrap_or_default();
    Ok(Import {
        path,
        is_recursive: false,
    })
});

impl_from_pest!(Element, |pest| {
    let pair = pest.next().ok_or(ConversionError::NoMatch)?;
    Ok(match pair.as_rule() {
        Rule::package_declaration => Element::Package(Package::from_pest(&mut pair.into_inner())?),
        Rule::part_definition
        | Rule::action_definition
        | Rule::requirement_definition
        | Rule::port_definition
        | Rule::item_definition
        | Rule::attribute_definition
        | Rule::concern_definition
        | Rule::case_definition
        | Rule::analysis_case_definition
        | Rule::verification_case_definition
        | Rule::use_case_definition
        | Rule::view_definition
        | Rule::viewpoint_definition
        | Rule::rendering_definition => {
            Element::Definition(Definition::from_pest(&mut pair.into_inner())?)
        }
        Rule::part_usage
        | Rule::action_usage
        | Rule::requirement_usage
        | Rule::port_usage
        | Rule::item_usage
        | Rule::attribute_usage
        | Rule::concern_usage
        | Rule::case_usage
        | Rule::view_usage => Element::Usage(Usage::from_pest(&mut pair.into_inner())?),
        Rule::comment_annotation => Element::Comment(Comment::from_pest(&mut pair.into_inner())?),
        Rule::import => Element::Import(Import::from_pest(&mut pair.into_inner())?),
        _ => return Err(ConversionError::NoMatch),
    })
});

impl_from_pest!(SysMLFile, |pest| {
    let mut elements = Vec::new();
    for pair in pest.clone() {
        if pair.as_rule() == Rule::namespace_element
            && let Ok(element) = Element::from_pest(&mut pair.into_inner())
        {
            elements.push(element);
        }
    }
    Ok(SysMLFile {
        namespace: None,
        elements,
    })
});
