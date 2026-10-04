fn main() {
    let root_dir = std::path::Path::new(".");
    let common_dir = root_dir.join("common");
    let php_dir = root_dir.join("php").join("src");
    let php_only_dir = root_dir.join("php_only").join("src");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(&php_dir);

    c_config.define("tree_sitter_php", "brokk_tree_sitter_php");
    c_config.define(
        "tree_sitter_php_external_scanner_create",
        "brokk_tree_sitter_php_external_scanner_create",
    );
    c_config.define(
        "tree_sitter_php_external_scanner_destroy",
        "brokk_tree_sitter_php_external_scanner_destroy",
    );
    c_config.define(
        "tree_sitter_php_external_scanner_scan",
        "brokk_tree_sitter_php_external_scanner_scan",
    );
    c_config.define(
        "tree_sitter_php_external_scanner_serialize",
        "brokk_tree_sitter_php_external_scanner_serialize",
    );
    c_config.define(
        "tree_sitter_php_external_scanner_deserialize",
        "brokk_tree_sitter_php_external_scanner_deserialize",
    );
    c_config.define("tree_sitter_php_only", "brokk_tree_sitter_php_only");
    c_config.define(
        "tree_sitter_php_only_external_scanner_create",
        "brokk_tree_sitter_php_only_external_scanner_create",
    );
    c_config.define(
        "tree_sitter_php_only_external_scanner_destroy",
        "brokk_tree_sitter_php_only_external_scanner_destroy",
    );
    c_config.define(
        "tree_sitter_php_only_external_scanner_scan",
        "brokk_tree_sitter_php_only_external_scanner_scan",
    );
    c_config.define(
        "tree_sitter_php_only_external_scanner_serialize",
        "brokk_tree_sitter_php_only_external_scanner_serialize",
    );
    c_config.define(
        "tree_sitter_php_only_external_scanner_deserialize",
        "brokk_tree_sitter_php_only_external_scanner_deserialize",
    );

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    println!("cargo:rerun-if-changed={}", common_dir.to_str().unwrap());

    for dir in &[php_dir, php_only_dir] {
        let parser_path = dir.join("parser.c");
        let scanner_path = dir.join("scanner.c");
        c_config.file(&parser_path);
        c_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    }

    c_config.compile("brokk-tree-sitter-php");
}
