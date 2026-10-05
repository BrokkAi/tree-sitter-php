//! This crate provides PHP language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [LANGUAGE_PHP][] constant to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! use tree_sitter::Parser;
//!
//! let code = r#"
//! <?php
//!   echo "Hello, World!";
//! ?>
//! "#;
//! let mut parser = Parser::new();
//! let language = brokk_tree_sitter_php::LANGUAGE_PHP;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading PHP parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn brokk_tree_sitter_php() -> *const ();
    fn brokk_tree_sitter_php_only() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for PHP.
///
/// [LanguageFn]: https://docs.rs/tree-sitter-language/*/tree_sitter_language/struct.LanguageFn.html
pub const LANGUAGE_PHP: LanguageFn = unsafe { LanguageFn::from_raw(brokk_tree_sitter_php) };

/// The tree-sitter [`LanguageFn`] for PHP-Only.
///
/// [LanguageFn]: https://docs.rs/tree-sitter-language/*/tree_sitter_language/struct.LanguageFn.html
pub const LANGUAGE_PHP_ONLY: LanguageFn =
    unsafe { LanguageFn::from_raw(brokk_tree_sitter_php_only) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const PHP_NODE_TYPES: &str = include_str!("../../php/src/node-types.json");
pub const PHP_ONLY_NODE_TYPES: &str = include_str!("../../php_only/src/node-types.json");

/// The syntax highlighting query for PHP.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The injection query for PHP.
pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");

/// The symbol tagging query for PHP.
pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_php_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE_PHP.into())
            .expect("Error loading PHP parser");

        let code = r#"<?php echo "Hello, World!";"#;

        let tree = parser.parse(code, None).unwrap();
        let root = tree.root_node();
        assert!(!root.has_error());
    }

    #[test]
    fn test_php_only_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE_PHP_ONLY.into())
            .expect("Error loading PHP-Only parser");

        let code = r#"echo "Hello, World!";"#;

        let tree = parser.parse(code, None).unwrap();
        let root = tree.root_node();
        assert!(!root.has_error());
    }
}

#[cfg(test)]
mod promotion_regressions {
    #[test]
    fn modern_php_parses_in_both_dialects() {
        let body = r#"namespace Demo;
class Example {
    public private(set) string $ordinary;
    public function __construct(
        public private(set) string $name,
        public protected(set) int $age = 0,
        protected private(set) ?string $alias = null,
        protected(set) string $implicit = '',
        #[Example] public private(set) readonly string $id = '',
        public private(set) string &$reference = '',
    ) { namespace\encode($name); namespace\Nested\encode($name); }
}"#;
        for (language, prefix) in [
            (super::LANGUAGE_PHP, "<?php "),
            (super::LANGUAGE_PHP_ONLY, ""),
        ] {
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(&language.into()).unwrap();
            let source = format!("{prefix}{body}");
            let tree = parser.parse(&source, None).unwrap();
            assert!(
                !tree.root_node().has_error(),
                "{}",
                tree.root_node().to_sexp()
            );
            for invalid in [
                "class X { public function __construct(public private(set) string) {} }",
                "class X { public function __construct(public private(set string $x) {} }",
            ] {
                let tree = parser.parse(format!("{prefix}{invalid}"), None).unwrap();
                assert!(tree.root_node().has_error(), "{invalid}");
            }
        }
    }
}

#[cfg(test)]
mod upstream_comparison {
    #[test]
    fn fork_coexists_with_upstream_and_repairs_promoted_visibility() {
        for (upstream, fixed, prefix) in [
            (
                upstream_tree_sitter_php::LANGUAGE_PHP,
                super::LANGUAGE_PHP,
                "<?php ",
            ),
            (
                upstream_tree_sitter_php::LANGUAGE_PHP_ONLY,
                super::LANGUAGE_PHP_ONLY,
                "",
            ),
        ] {
            let code = format!("{prefix}class X {{ public function __construct(public private(set) string $name) {{}} }}");
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(&upstream.into()).unwrap();
            assert!(parser.parse(&code, None).unwrap().root_node().has_error());
            parser.set_language(&fixed.into()).unwrap();
            assert!(!parser.parse(&code, None).unwrap().root_node().has_error());
        }
    }

    #[test]
    fn clone_with_arguments_parse_as_a_complete_declaration_in_both_dialects() {
        let body = "class X { public string $name; public function f(): static { return clone($this, ['name' => 'x']); } }";
        for (upstream, fixed, prefix) in [
            (
                upstream_tree_sitter_php::LANGUAGE_PHP,
                super::LANGUAGE_PHP,
                "<?php ",
            ),
            (
                upstream_tree_sitter_php::LANGUAGE_PHP_ONLY,
                super::LANGUAGE_PHP_ONLY,
                "",
            ),
        ] {
            let source = format!("{prefix}{body}");
            let mut parser = tree_sitter::Parser::new();
            parser.set_language(&upstream.into()).unwrap();
            assert!(parser.parse(&source, None).unwrap().root_node().has_error());

            parser.set_language(&fixed.into()).unwrap();
            let tree = parser.parse(&source, None).unwrap();
            let root = tree.root_node();
            assert!(!root.has_error(), "{}", root.to_sexp());
            assert_eq!(root.start_byte(), 0);
            assert_eq!(root.end_byte(), source.len());
            let declaration = (0..root.named_child_count())
                .map(|index| root.named_child(index).unwrap())
                .find(|node| node.kind() == "class_declaration")
                .unwrap();
            assert_eq!(declaration.kind(), "class_declaration");
            assert_eq!(declaration.end_byte(), source.len());

            for invalid in [
                "class X { public function f(): static { return clone($this, , ['name' => 'x']); } }",
                "class X { public function f(): static { return clone($this, withProperties: ); } }",
            ] {
                let invalid_source = format!("{prefix}{invalid}");
                let tree = parser.parse(&invalid_source, None).unwrap();
                assert!(tree.root_node().has_error(), "{invalid}");
            }
        }
    }
}
