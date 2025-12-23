use crate::semantic::symbol_table::Symbol;
use crate::syntax::kerml::ast::{Classifier, Element, NamespaceDeclaration, Package};

use crate::semantic::adapters::KermlAdapter;

impl<'a> KermlAdapter<'a> {
    pub(super) fn visit_namespace(&mut self, namespace: &NamespaceDeclaration) {
        let qualified_name = self.qualified_name(&namespace.name);
        let scope_id = self.symbol_table.current_scope_id();
        let symbol = Symbol::Package {
            name: namespace.name.clone(),
            qualified_name,
            scope_id,
            source_file: self.symbol_table.current_file().map(String::from),
            span: namespace.span,
            references: Vec::new(),
        };
        self.insert_symbol(namespace.name.clone(), symbol);
        self.enter_namespace(namespace.name.clone());
    }

    pub(super) fn visit_package(&mut self, package: &Package) {
        if let Some(name) = &package.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();
            let symbol = Symbol::Package {
                name: name.clone(),
                qualified_name,
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: package.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);
            self.enter_namespace(name.clone());

            for element in &package.elements {
                self.visit_element(element);
            }

            self.exit_namespace();
        }
    }

    pub(super) fn visit_classifier(&mut self, classifier: &Classifier) {
        if let Some(name) = &classifier.name {
            let qualified_name = self.qualified_name(name);
            let scope_id = self.symbol_table.current_scope_id();
            let kind_str = format!("{:?}", classifier.kind);
            let symbol = Symbol::Classifier {
                name: name.clone(),
                qualified_name,
                kind: kind_str,
                is_abstract: classifier.is_abstract,
                scope_id,
                source_file: self.symbol_table.current_file().map(String::from),
                span: classifier.span,
                references: Vec::new(),
            };
            self.insert_symbol(name.clone(), symbol);
            self.enter_namespace(name.clone());

            // TODO: Process classifier members

            self.exit_namespace();
        }
    }

    pub(super) fn visit_element(&mut self, element: &Element) {
        match element {
            Element::Package(package) => self.visit_package(package),
            Element::Classifier(classifier) => self.visit_classifier(classifier),
            Element::Feature(_) => {
                // Features at top level are less common
            }
            Element::Import(_) | Element::Annotation(_) | Element::Comment(_) => {
                // Skip for now
            }
        }
    }
}
