use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../lib/clientele/src");
    let mut files = vec!["clap/help_styles.rs", "options.rs", "options/sort.rs"];
    if env::var_os("CARGO_FEATURE_TRACING").is_some() {
        files.push("tracing.rs");
    }
    let mut examples = String::new();
    for file in files {
        let path = root.join(file);
        println!("cargo:rerun-if-changed={}", path.display());
        examples.push_str(&format!("# {file}\n\n"));
        let source = fs::read_to_string(path).unwrap();
        let mut in_example = false;
        for line in source.lines() {
            let Some(comment) = line.trim_start().strip_prefix("///") else {
                continue;
            };
            let comment = comment.strip_prefix(' ').unwrap_or(comment);
            if comment.starts_with("```") {
                in_example = !in_example;
                examples.push_str(comment);
                examples.push_str("\n\n");
            } else if in_example {
                examples.push_str(comment);
                examples.push('\n');
            }
        }
        assert!(!in_example, "unclosed example in {file}");
    }
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("examples.md");
    fs::write(output, examples).unwrap();
}
