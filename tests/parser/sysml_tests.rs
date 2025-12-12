#![allow(clippy::unwrap_used)]

use pest::Parser;
use rstest::rstest;
use syster::parser::{SysMLParser, sysml::Rule};

#[test]
fn test_parse_simple_identifier() {
    let input = "myVar";
    let result = SysMLParser::parse(Rule::identifier, input);

    assert!(
        result.is_ok(),
        "Failed to parse identifier: {:?}",
        result.err()
    );
    let pairs = result.unwrap();
    let identifier = pairs.into_iter().next().unwrap();
    assert_eq!(identifier.as_str(), "myVar");
}

#[rstest]
#[case("about")]
#[case("abstract")]
#[case("accept")]
#[case("action")]
#[case("actor")]
#[case("after")]
#[case("alias")]
#[case("all")]
#[case("allocate")]
#[case("allocation")]
#[case("analysis")]
#[case("and")]
#[case("as")]
#[case("assert")]
#[case("assign")]
#[case("assume")]
#[case("at")]
#[case("attribute")]
#[case("bind")]
#[case("binding")]
#[case("by")]
#[case("calc")]
#[case("case")]
#[case("comment")]
#[case("concern")]
#[case("connect")]
#[case("connection")]
#[case("constraint")]
#[case("crosses")]
#[case("decide")]
#[case("def")]
#[case("default")]
#[case("defined")]
#[case("dependency")]
#[case("derived")]
#[case("do")]
#[case("doc")]
#[case("else")]
#[case("end")]
#[case("entry")]
#[case("enum")]
#[case("event")]
#[case("exhibit")]
#[case("exit")]
#[case("expose")]
#[case("false")]
#[case("filter")]
#[case("first")]
#[case("flow")]
#[case("for")]
#[case("fork")]
#[case("frame")]
#[case("from")]
#[case("hastype")]
#[case("if")]
#[case("implies")]
#[case("import")]
#[case("in")]
#[case("include")]
#[case("individual")]
#[case("inout")]
#[case("interface")]
#[case("istype")]
#[case("item")]
#[case("join")]
#[case("language")]
#[case("library")]
#[case("locale")]
#[case("loop")]
#[case("merge")]
#[case("message")]
#[case("meta")]
#[case("metadata")]
#[case("nonunique")]
#[case("not")]
#[case("null")]
#[case("objective")]
#[case("occurrence")]
#[case("of")]
#[case("or")]
#[case("ordered")]
#[case("out")]
#[case("package")]
#[case("parallel")]
#[case("part")]
#[case("perform")]
#[case("port")]
#[case("private")]
#[case("protected")]
#[case("public")]
#[case("readonly")]
#[case("redefines")]
#[case("ref")]
#[case("references")]
#[case("render")]
#[case("rendering")]
#[case("rep")]
#[case("require")]
#[case("requirement")]
#[case("return")]
#[case("satisfy")]
#[case("send")]
#[case("snapshot")]
#[case("specializes")]
#[case("stakeholder")]
#[case("standard")]
#[case("state")]
#[case("subject")]
#[case("subsets")]
#[case("succession")]
#[case("terminate")]
#[case("then")]
#[case("timeslice")]
#[case("to")]
#[case("transition")]
#[case("true")]
#[case("until")]
#[case("use")]
#[case("variant")]
#[case("variation")]
#[case("verification")]
#[case("verify")]
#[case("via")]
#[case("view")]
#[case("viewpoint")]
#[case("when")]
#[case("while")]
#[case("xor")]
fn test_parse_keywords(#[case] keyword: &str) {
    let pairs = SysMLParser::parse(Rule::keyword, keyword).unwrap();
    let parsed = pairs.into_iter().next().unwrap();
    assert_eq!(parsed.as_str(), keyword);
}

#[test]
fn test_parse_line_comment() {
    let input = "// this is a comment";
    let result = SysMLParser::parse(Rule::line_comment, input);

    assert!(
        result.is_ok(),
        "Failed to parse line comment: {:?}",
        result.err()
    );
    let pairs = result.unwrap();
    let comment = pairs.into_iter().next().unwrap();
    assert_eq!(comment.as_str(), "// this is a comment");
}

#[test]
fn test_parse_block_comment() {
    let input = "/* block comment */";
    let result = SysMLParser::parse(Rule::block_comment, input);

    assert!(
        result.is_ok(),
        "Failed to parse block comment: {:?}",
        result.err()
    );
    let pairs = result.unwrap();
    let comment = pairs.into_iter().next().unwrap();
    assert_eq!(comment.as_str(), "/* block comment */");
}

#[test]
fn test_parse_multiline_block_comment() {
    let input = "/* line 1\nline 2\nline 3 */";
    let result = SysMLParser::parse(Rule::block_comment, input);

    assert!(
        result.is_ok(),
        "Failed to parse multiline block comment: {:?}",
        result.err()
    );
    let pairs = result.unwrap();
    let comment = pairs.into_iter().next().unwrap();
    assert_eq!(comment.as_str(), "/* line 1\nline 2\nline 3 */");
}
