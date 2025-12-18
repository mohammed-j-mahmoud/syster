#![allow(clippy::unwrap_used)]

use pest::Parser;
use syster::parser::{SysMLParser, sysml::Rule};

#[test]
fn test_chained_member_access() {
    let input = "fn.samples.domainValue";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse chained member access: {:?}",
        result.err()
    );
}

#[test]
fn test_instantiation_expression_with_args() {
    let input = "new SampledFunction(samples = values)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse instantiation expression: {:?}",
        result.err()
    );
}

#[test]
fn test_instantiation_expression_positional() {
    let input = "new SamplePair(x, y)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse instantiation with positional args: {:?}",
        result.err()
    );
}

#[test]
fn test_arrow_invocation_with_block() {
    let input = "list->select { in i; true }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse arrow invocation with block: {:?}",
        result.err()
    );
}

#[test]
fn test_arrow_invocation_with_block_then_index() {
    let input = "list->select { in i; true }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse arrow invocation with block followed by indexing: {:?}",
        result.err()
    );
}

#[test]
fn test_typed_parameter_in_lambda() {
    let input = "list->select { in i : Positive; domainValues#(i) <= value }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse lambda with typed parameter: {:?}",
        result.err()
    );
}

#[test]
fn test_typed_parameter_in_lambda_then_index() {
    let input = "list->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse lambda with typed parameter followed by indexing: {:?}",
        result.err()
    );
}

#[test]
fn test_full_sampled_functions_expression() {
    let input =
        "(1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse full SampledFunctions expression: {:?}",
        result.err()
    );
}

#[test]
fn test_nested_instantiation_in_collect() {
    let input = "domainValues->collect { in x; new SamplePair(x, calculation(x)) }";
    let result = SysMLParser::parse(Rule::primary_expression, input);

    assert!(
        result.is_ok(),
        "Failed to parse collect with nested instantiation: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_complex_initializer() {
    let input = "attribute index : Positive[0..1] = (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1);";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute with complex initializer: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_without_type() {
    let input = "attribute index = (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1);";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute without type: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_in_package_context() {
    let input =
        "package Test { attribute index = list->select { in i : Positive; vals#(i) <= v }#(1); }";
    let result = SysMLParser::parse(Rule::package, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute in package: {:?}",
        result.err()
    );
}

#[test]
fn test_attribute_with_simple_type() {
    let input = "attribute index : Positive[0..1];";
    let result = SysMLParser::parse(Rule::attribute_usage, input);

    assert!(
        result.is_ok(),
        "Failed to parse attribute with simple type: {:?}",
        result.err()
    );
}

#[test]
fn test_feature_value_with_lambda() {
    use pest::Parser;
    use syster::parser::{SysMLParser, sysml::Rule};

    let input =
        "= (1..size(domainValues))->select { in i : Positive; domainValues#(i) <= value }#(1)";
    let result = SysMLParser::parse(Rule::feature_value, input);

    assert!(
        result.is_ok(),
        "Failed to parse feature_value: {:?}",
        result.err()
    );
}
