#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use super::*;
use crate::semantic::symbol_table::{ClassifierKind, DefinitionKind, Symbol, UsageKind};

#[test]
fn test_analyzer_creation() {
    let analyzer = SemanticAnalyzer::new();
    assert_eq!(analyzer.symbol_table().current_scope_id(), 0);
}

#[test]
fn test_analyze_empty_table() {
    let analyzer = SemanticAnalyzer::new();
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyze_with_valid_symbols() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "MyPackage".to_string(),
            Symbol::Package {
                name: "MyPackage".to_string(),
                qualified_name: "MyPackage".to_string(),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_error_handling() {
    let table = SymbolTable::new();
    let mut context = AnalysisContext::new(&table);

    assert!(!context.has_errors());

    context.add_error(SemanticError::undefined_reference("Test".to_string()));

    assert!(context.has_errors());
    assert_eq!(context.errors.len(), 1);
}

#[test]
fn test_context_into_result_success() {
    let table = SymbolTable::new();
    let context = AnalysisContext::new(&table);

    let result: SemanticResult<i32> = context.into_result(42);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_context_into_result_error() {
    let table = SymbolTable::new();
    let mut context = AnalysisContext::new(&table);

    context.add_error(SemanticError::undefined_reference("Test".to_string()));

    let result: SemanticResult<i32> = context.into_result(42);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().len(), 1);
}

#[test]
fn test_context_multiple_errors() {
    let table = SymbolTable::new();
    let mut context = AnalysisContext::new(&table);

    context.add_error(SemanticError::undefined_reference("Test1".to_string()));
    context.add_error(SemanticError::undefined_reference("Test2".to_string()));
    context.add_error(SemanticError::invalid_type("BadType".to_string()));

    assert!(context.has_errors());
    assert_eq!(context.errors.len(), 3);

    let result: SemanticResult<()> = context.into_result(());
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 3);
}

#[test]
fn test_analyzer_with_multiple_symbols() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Pkg".to_string(),
            Symbol::Package {
                name: "Pkg".to_string(),
                qualified_name: "Pkg".to_string(),
            },
        )
        .unwrap();

    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                name: "MyClass".to_string(),
                qualified_name: "MyClass".to_string(),
                kind: ClassifierKind::Class,
                is_abstract: false,
            },
        )
        .unwrap();

    table
        .insert(
            "MyDef".to_string(),
            Symbol::Definition {
                name: "MyDef".to_string(),
                qualified_name: "MyDef".to_string(),
                kind: DefinitionKind::Part,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyzer_with_nested_scopes() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Root".to_string(),
            Symbol::Package {
                name: "Root".to_string(),
                qualified_name: "Root".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "Child".to_string(),
            Symbol::Package {
                name: "Child".to_string(),
                qualified_name: "Root::Child".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "GrandChild".to_string(),
            Symbol::Classifier {
                name: "GrandChild".to_string(),
                qualified_name: "Root::Child::GrandChild".to_string(),
                kind: ClassifierKind::Class,
                is_abstract: false,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_resolver_access() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "Test".to_string(),
            Symbol::Package {
                name: "Test".to_string(),
                qualified_name: "Test".to_string(),
            },
        )
        .unwrap();

    let context = AnalysisContext::new(&table);
    let resolved = context.resolver.resolve("Test");
    assert!(resolved.is_some());

    let Some(Symbol::Package { name, .. }) = resolved else {
        panic!("Expected Package symbol, got: {resolved:?}");
    };
    assert_eq!(name, "Test");
}

#[test]
fn test_analyzer_default() {
    let analyzer = SemanticAnalyzer::default();
    assert_eq!(analyzer.symbol_table().current_scope_id(), 0);
}

#[test]
fn test_analyzer_symbol_table_access() {
    let mut analyzer = SemanticAnalyzer::new();

    analyzer
        .symbol_table_mut()
        .insert(
            "NewSymbol".to_string(),
            Symbol::Package {
                name: "NewSymbol".to_string(),
                qualified_name: "NewSymbol".to_string(),
            },
        )
        .unwrap();

    assert!(analyzer.symbol_table().lookup("NewSymbol").is_some());
}

#[test]
fn test_analyze_with_all_symbol_types() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Pkg".to_string(),
            Symbol::Package {
                name: "Pkg".to_string(),
                qualified_name: "Pkg".to_string(),
            },
        )
        .unwrap();

    table
        .insert(
            "MyClass".to_string(),
            Symbol::Classifier {
                name: "MyClass".to_string(),
                qualified_name: "MyClass".to_string(),
                kind: ClassifierKind::Behavior,
                is_abstract: true,
            },
        )
        .unwrap();

    table
        .insert(
            "myFeature".to_string(),
            Symbol::Feature {
                name: "myFeature".to_string(),
                qualified_name: "MyClass::myFeature".to_string(),
                feature_type: Some("Integer".to_string()),
            },
        )
        .unwrap();

    table
        .insert(
            "MyDef".to_string(),
            Symbol::Definition {
                name: "MyDef".to_string(),
                qualified_name: "MyDef".to_string(),
                kind: DefinitionKind::Requirement,
            },
        )
        .unwrap();

    table
        .insert(
            "MyUsage".to_string(),
            Symbol::Usage {
                name: "MyUsage".to_string(),
                qualified_name: "MyUsage".to_string(),
                kind: UsageKind::Action,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_into_result_with_unit() {
    let table = SymbolTable::new();
    let context = AnalysisContext::new(&table);

    let result: SemanticResult<()> = context.into_result(());
    assert!(result.is_ok());
}

#[test]
fn test_context_error_accumulation() {
    let table = SymbolTable::new();
    let mut context = AnalysisContext::new(&table);

    for i in 0..10 {
        context.add_error(SemanticError::undefined_reference(format!("Symbol{}", i)));
    }

    assert_eq!(context.errors.len(), 10);
    let result: SemanticResult<()> = context.into_result(());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().len(), 10);
}

#[test]
fn test_analyzer_immutable_access() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "Test".to_string(),
            Symbol::Package {
                name: "Test".to_string(),
                qualified_name: "Test".to_string(),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let sym_table = analyzer.symbol_table();
    assert!(sym_table.lookup("Test").is_some());
}

#[test]
fn test_analyze_idempotent() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "Test".to_string(),
            Symbol::Package {
                name: "Test".to_string(),
                qualified_name: "Test".to_string(),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);

    let result1 = analyzer.analyze();
    let result2 = analyzer.analyze();

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[test]
fn test_context_no_errors_initially() {
    let table = SymbolTable::new();
    let context = AnalysisContext::new(&table);

    assert!(!context.has_errors());
    assert_eq!(context.errors.len(), 0);
}

#[test]
fn test_analyzer_with_features() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "feature1".to_string(),
            Symbol::Feature {
                name: "feature1".to_string(),
                qualified_name: "feature1".to_string(),
                feature_type: None,
            },
        )
        .unwrap();

    table
        .insert(
            "feature2".to_string(),
            Symbol::Feature {
                name: "feature2".to_string(),
                qualified_name: "feature2".to_string(),
                feature_type: Some("String".to_string()),
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyzer_with_different_classifier_kinds() {
    let mut table = SymbolTable::new();

    for (idx, kind) in [
        ClassifierKind::Type,
        ClassifierKind::Class,
        ClassifierKind::DataType,
        ClassifierKind::Structure,
        ClassifierKind::Association,
        ClassifierKind::Behavior,
        ClassifierKind::Function,
    ]
    .iter()
    .enumerate()
    {
        table
            .insert(
                format!("Classifier{}", idx),
                Symbol::Classifier {
                    name: format!("Classifier{}", idx),
                    qualified_name: format!("Classifier{}", idx),
                    kind: kind.clone(),
                    is_abstract: false,
                },
            )
            .unwrap();
    }

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyzer_with_different_definition_kinds() {
    let mut table = SymbolTable::new();

    for (idx, kind) in [
        DefinitionKind::Part,
        DefinitionKind::Port,
        DefinitionKind::Action,
        DefinitionKind::State,
        DefinitionKind::Requirement,
        DefinitionKind::Item,
    ]
    .iter()
    .enumerate()
    {
        table
            .insert(
                format!("Def{}", idx),
                Symbol::Definition {
                    name: format!("Def{}", idx),
                    qualified_name: format!("Def{}", idx),
                    kind: kind.clone(),
                },
            )
            .unwrap();
    }

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_analyzer_with_different_usage_kinds() {
    let mut table = SymbolTable::new();

    for (idx, kind) in [
        UsageKind::Part,
        UsageKind::Port,
        UsageKind::Action,
        UsageKind::State,
        UsageKind::Requirement,
    ]
    .iter()
    .enumerate()
    {
        table
            .insert(
                format!("Usage{}", idx),
                Symbol::Usage {
                    name: format!("Usage{}", idx),
                    qualified_name: format!("Usage{}", idx),
                    kind: kind.clone(),
                },
            )
            .unwrap();
    }

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_with_qualified_name_resolution() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "A".to_string(),
            Symbol::Package {
                name: "A".to_string(),
                qualified_name: "A".to_string(),
            },
        )
        .unwrap();

    table.enter_scope();
    table
        .insert(
            "B".to_string(),
            Symbol::Package {
                name: "B".to_string(),
                qualified_name: "A::B".to_string(),
            },
        )
        .unwrap();

    let context = AnalysisContext::new(&table);
    let resolved = context.resolver.resolve("A::B");
    assert!(resolved.is_some());
}

#[test]
fn test_analyzer_with_abstract_classifiers() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "Abstract1".to_string(),
            Symbol::Classifier {
                name: "Abstract1".to_string(),
                qualified_name: "Abstract1".to_string(),
                kind: ClassifierKind::Class,
                is_abstract: true,
            },
        )
        .unwrap();

    table
        .insert(
            "Concrete1".to_string(),
            Symbol::Classifier {
                name: "Concrete1".to_string(),
                qualified_name: "Concrete1".to_string(),
                kind: ClassifierKind::Class,
                is_abstract: false,
            },
        )
        .unwrap();

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_into_result_preserves_value() {
    let table = SymbolTable::new();
    let context = AnalysisContext::new(&table);

    let test_value = vec![1, 2, 3, 4, 5];
    let result: SemanticResult<Vec<i32>> = context.into_result(test_value.clone());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), test_value);
}

#[test]
fn test_context_symbol_table_reference() {
    let mut table = SymbolTable::new();
    table
        .insert(
            "Test".to_string(),
            Symbol::Package {
                name: "Test".to_string(),
                qualified_name: "Test".to_string(),
            },
        )
        .unwrap();

    let context = AnalysisContext::new(&table);
    let lookup_result = context.symbol_table.lookup("Test");
    assert!(lookup_result.is_some());
}

#[test]
fn test_analyzer_with_deeply_nested_structure() {
    let mut table = SymbolTable::new();

    table
        .insert(
            "L0".to_string(),
            Symbol::Package {
                name: "L0".to_string(),
                qualified_name: "L0".to_string(),
            },
        )
        .unwrap();

    for i in 1..=5 {
        table.enter_scope();
        let parent = if i == 1 {
            "L0".to_string()
        } else {
            format!(
                "L0::{}",
                (1..i)
                    .map(|j| format!("L{}", j))
                    .collect::<Vec<_>>()
                    .join("::")
            )
        };
        table
            .insert(
                format!("L{}", i),
                Symbol::Package {
                    name: format!("L{}", i),
                    qualified_name: format!("{}::L{}", parent, i),
                },
            )
            .unwrap();
    }

    let analyzer = SemanticAnalyzer::with_symbol_table(table);
    let result = analyzer.analyze();
    assert!(result.is_ok());
}

#[test]
fn test_context_error_types_variety() {
    let table = SymbolTable::new();
    let mut context = AnalysisContext::new(&table);

    context.add_error(SemanticError::undefined_reference("Ref1".to_string()));
    context.add_error(SemanticError::invalid_type("Type1".to_string()));
    context.add_error(SemanticError::duplicate_definition(
        "Dup1".to_string(),
        None,
    ));
    context.add_error(SemanticError::type_mismatch(
        "Int".to_string(),
        "Str".to_string(),
        "test".to_string(),
    ));
    context.add_error(SemanticError::circular_dependency(vec![
        "A".to_string(),
        "B".to_string(),
    ]));

    assert_eq!(context.errors.len(), 5);
}

#[test]
fn test_analyzer_table_mutation_after_creation() {
    let mut analyzer = SemanticAnalyzer::new();

    let table_mut = analyzer.symbol_table_mut();
    table_mut
        .insert(
            "Added".to_string(),
            Symbol::Package {
                name: "Added".to_string(),
                qualified_name: "Added".to_string(),
            },
        )
        .unwrap();

    let result = analyzer.analyze();
    assert!(result.is_ok());
}
