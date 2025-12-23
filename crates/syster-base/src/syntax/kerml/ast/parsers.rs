use super::types::{Classifier, Feature};
use super::utils::{
    extract_direction, extract_flags, find_identifier_span, find_name, to_classifier_kind,
};
use crate::parser::kerml::Rule;
use from_pest::{ConversionError, Void};
use pest::iterators::Pair;

/// Parse a classifier from a pest pair
pub fn parse_classifier(pair: Pair<Rule>) -> Result<Classifier, ConversionError<Void>> {
    let kind = to_classifier_kind(pair.as_rule())?;
    let pairs: Vec<_> = pair.into_inner().collect();

    // Find the identifier and its span
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    Ok(Classifier {
        kind,
        is_abstract: pairs.iter().any(|p| p.as_rule() == Rule::abstract_marker),
        name,
        body: Vec::new(),
        span,
    })
}

/// Parse a feature from a pest pair
pub fn parse_feature(pair: Pair<Rule>) -> Feature {
    let pairs: Vec<_> = pair.into_inner().collect();
    let (is_readonly, is_derived) = extract_flags(&pairs);

    // Find the identifier and its span
    let (name, span) = find_identifier_span(pairs.iter().cloned());
    let name = name.or_else(|| find_name(pairs.iter().cloned()));

    Feature {
        name,
        direction: extract_direction(&pairs),
        is_readonly,
        is_derived,
        body: Vec::new(),
        span,
    }
}
