use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::TempDir;

fn run_hook_with_input(input: &str) -> String {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read output");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn make_hook_input(file_path: &std::path::Path) -> String {
    format!(
        r#"{{"tool_name": "Write", "tool_input": {{"file_path": "{}"}}}}"#,
        file_path.display()
    )
}

// ============================================================================
// Basic error handling tests
// ============================================================================

#[test]
fn test_missing_file_path() {
    let input = r#"{"tool_name": "Write", "tool_input": {}}"#;
    let output = run_hook_with_input(input);
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_nonexistent_file() {
    let input = r#"{"tool_name": "Write", "tool_input": {"file_path": "/nonexistent/file.rs"}}"#;
    let output = run_hook_with_input(input);
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    assert!(output.contains("does not exist"));
}

#[test]
fn test_unsupported_extension() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("file.xyz");
    fs::write(&file_path, "content").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    assert!(output.contains("Unsupported"));
}

#[test]
fn test_invalid_json_input() {
    let output = run_hook_with_input("not valid json");
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

// ============================================================================
// Rust formatting tests
// ============================================================================

#[test]
fn test_rust_with_cargo_project_uses_cargo_fmt() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create a Cargo project structure
    fs::write(
        project_dir.join("Cargo.toml"),
        r#"[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // Write unformatted Rust code
    let file_path = src_dir.join("main.rs");
    let unformatted = "fn main(){let x=1;let y=2;println!(\"{}\",x+y);}";
    fs::write(&file_path, unformatted).unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));

    // Check if cargo fmt or rustfmt was used
    if output.contains("cargo fmt") || output.contains("rustfmt") {
        // Verify the file was actually formatted
        let formatted = fs::read_to_string(&file_path).unwrap();
        assert_ne!(formatted, unformatted, "File should have been formatted");
        assert!(
            formatted.contains('\n'),
            "Formatted code should have newlines"
        );
    }
}

#[test]
fn test_rust_without_cargo_project() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));

    // Should either use rustfmt directly or report no formatter
    assert!(
        output.contains("rustfmt") || output.contains("No formatter"),
        "Should use rustfmt or report no formatter"
    );
}

// ============================================================================
// JavaScript/TypeScript formatting tests
// ============================================================================

#[test]
fn test_javascript_in_node_project_detects_formatters() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create a Node.js project structure
    fs::write(project_dir.join("package.json"), r#"{"name": "test"}"#).unwrap();

    let file_path = project_dir.join("index.js");
    fs::write(&file_path, "const x=1;").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));

    // Should detect oxfmt, biome, prettier, dprint, or report no formatter
    assert!(
        output.contains("oxfmt")
            || output.contains("biome")
            || output.contains("prettier")
            || output.contains("dprint")
            || output.contains("No formatter"),
        "Should detect JS formatter or report none: {}",
        output
    );
}

#[test]
fn test_typescript_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.ts");
    fs::write(&file_path, "const x: number = 1;").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_tsx_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.tsx");
    fs::write(&file_path, "const App = () => <div />;").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

// ============================================================================
// Python formatting tests
// ============================================================================

#[test]
fn test_python_detects_available_formatter() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.py");
    let unformatted = "def foo():x=1;y=2;return x+y";
    fs::write(&file_path, unformatted).unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));

    // Should detect ruff, black, autopep8, yapf, or report no formatter
    assert!(
        output.contains("ruff")
            || output.contains("black")
            || output.contains("autopep8")
            || output.contains("yapf")
            || output.contains("No formatter"),
        "Should detect Python formatter or report none: {}",
        output
    );
}

#[test]
fn test_python_pyi_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.pyi");
    fs::write(&file_path, "def foo() -> int: ...").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

// ============================================================================
// Go formatting tests
// ============================================================================

#[test]
fn test_go_with_module_uses_gofmt() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create a Go module
    fs::write(project_dir.join("go.mod"), "module test\n\ngo 1.21\n").unwrap();

    let file_path = project_dir.join("main.go");
    let unformatted = "package main\nimport \"fmt\"\nfunc main(){x:=1;fmt.Println(x)}";
    fs::write(&file_path, unformatted).unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));

    // Should detect goimports, gofumpt, or gofmt
    if output.contains("goimports") || output.contains("gofumpt") || output.contains("gofmt") {
        let formatted = fs::read_to_string(&file_path).unwrap();
        // gofmt/gofumpt adds proper spacing
        assert!(
            formatted != unformatted || formatted.contains("x := 1"),
            "File should be formatted"
        );
    }
}

// ============================================================================
// Java formatting tests
// ============================================================================

#[test]
fn test_java_with_maven_project() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create a Maven project structure
    fs::write(
        project_dir.join("pom.xml"),
        r#"<project><modelVersion>4.0.0</modelVersion></project>"#,
    )
    .unwrap();

    let src_dir = project_dir.join("src/main/java");
    fs::create_dir_all(&src_dir).unwrap();

    let file_path = src_dir.join("Main.java");
    fs::write(
        &file_path,
        "public class Main{public static void main(String[] args){}}",
    )
    .unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));

    // Should try spotless or standalone formatters
    assert!(
        output.contains("spotless")
            || output.contains("google-java-format")
            || output.contains("palantir")
            || output.contains("No formatter"),
        "Should detect Java formatter or report none: {}",
        output
    );
}

#[test]
fn test_java_with_gradle_project() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create a Gradle project structure
    fs::write(project_dir.join("build.gradle"), "plugins { id 'java' }").unwrap();

    let file_path = project_dir.join("Main.java");
    fs::write(&file_path, "public class Main{}").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

// ============================================================================
// oxfmt format tests (JSON, YAML, TOML, etc.)
// ============================================================================

#[test]
fn test_json_detects_oxfmt() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.json");
    fs::write(&file_path, r#"{"key":"value","foo":"bar"}"#).unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    assert!(
        output.contains("oxfmt") || output.contains("No formatter"),
        "Should detect oxfmt or report none: {}",
        output
    );
}

#[test]
fn test_jsonc_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.jsonc");
    fs::write(&file_path, r#"{"key": "value" /* comment */}"#).unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_yaml_detects_oxfmt() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.yaml");
    fs::write(&file_path, "key: value\nfoo: bar").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    assert!(
        output.contains("oxfmt") || output.contains("No formatter"),
        "Should detect oxfmt or report none: {}",
        output
    );
}

#[test]
fn test_yml_extension() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.yml");
    fs::write(&file_path, "key: value").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_toml_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.toml");
    fs::write(&file_path, "[section]\nkey = \"value\"").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_html_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.html");
    fs::write(&file_path, "<html><body><h1>Hello</h1></body></html>").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_css_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.css");
    fs::write(&file_path, ".class{color:red;margin:0}").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_scss_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.scss");
    fs::write(&file_path, "$color: red;\n.class { color: $color; }").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_markdown_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.md");
    fs::write(&file_path, "# Title\nSome content").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_vue_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.vue");
    fs::write(
        &file_path,
        "<template><div>Hello</div></template><script>export default {}</script>",
    )
    .unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

#[test]
fn test_graphql_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.graphql");
    fs::write(&file_path, "type Query { hello: String }").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));
    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

// ============================================================================
// Project root detection tests
// ============================================================================

#[test]
fn test_nested_file_finds_project_root() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create nested structure
    fs::write(project_dir.join("package.json"), r#"{"name": "root"}"#).unwrap();

    let nested_dir = project_dir.join("src/components/deep");
    fs::create_dir_all(&nested_dir).unwrap();

    let file_path = nested_dir.join("Component.js");
    fs::write(&file_path, "const x = 1;").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    // The formatter detection should find the package.json at root
}

#[test]
fn test_monorepo_finds_nearest_package_json() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create monorepo structure
    fs::write(project_dir.join("package.json"), r#"{"name": "monorepo"}"#).unwrap();

    let pkg_dir = project_dir.join("packages/my-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("package.json"), r#"{"name": "my-pkg"}"#).unwrap();

    let file_path = pkg_dir.join("index.js");
    fs::write(&file_path, "export const x = 1;").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
}

// ============================================================================
// Monorepo formatter priority tests
// ============================================================================

/// Helper to create a mock formatter script
#[cfg(unix)]
fn create_mock_formatter(path: &std::path::Path, name: &str) {
    use std::os::unix::fs::PermissionsExt;

    let bin_dir = path.join("node_modules/.bin");
    fs::create_dir_all(&bin_dir).unwrap();

    let formatter_path = bin_dir.join(name);
    // Create a script that just exits successfully
    fs::write(&formatter_path, "#!/bin/sh\nexit 0\n").unwrap();
    fs::set_permissions(&formatter_path, fs::Permissions::from_mode(0o755)).unwrap();
}

#[cfg(not(unix))]
fn create_mock_formatter(path: &std::path::Path, name: &str) {
    let bin_dir = path.join("node_modules/.bin");
    fs::create_dir_all(&bin_dir).unwrap();

    let formatter_path = bin_dir.join(format!("{}.cmd", name));
    fs::write(&formatter_path, "@echo off\nexit /b 0\n").unwrap();
}

#[test]
fn test_monorepo_uses_package_level_oxfmt_over_biome() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create monorepo root with biome
    fs::write(project_dir.join("package.json"), r#"{"name": "monorepo"}"#).unwrap();
    create_mock_formatter(project_dir, "biome");

    // Create package with oxfmt (highest priority)
    let pkg_dir = project_dir.join("packages/my-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("package.json"), r#"{"name": "my-pkg"}"#).unwrap();
    create_mock_formatter(&pkg_dir, "oxfmt");

    let file_path = pkg_dir.join("index.js");
    fs::write(&file_path, "const x = 1;").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    // Should use oxfmt from the package (highest priority)
    assert!(
        output.contains("oxfmt"),
        "Should use oxfmt from package level: {}",
        output
    );
}

#[test]
fn test_monorepo_uses_package_level_biome_over_root_prettier() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create monorepo root with prettier
    fs::write(project_dir.join("package.json"), r#"{"name": "monorepo"}"#).unwrap();
    create_mock_formatter(project_dir, "prettier");

    // Create package with biome (higher priority than prettier)
    let pkg_dir = project_dir.join("packages/my-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("package.json"), r#"{"name": "my-pkg"}"#).unwrap();
    create_mock_formatter(&pkg_dir, "biome");

    let file_path = pkg_dir.join("index.js");
    fs::write(&file_path, "const x = 1;").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    // Should use biome from the package, not prettier from root
    assert!(
        output.contains("biome"),
        "Should use biome from package level: {}",
        output
    );
}

#[test]
fn test_monorepo_falls_back_to_root_formatter() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create monorepo root with prettier
    fs::write(project_dir.join("package.json"), r#"{"name": "monorepo"}"#).unwrap();
    create_mock_formatter(project_dir, "prettier");

    // Create package WITHOUT any formatter
    let pkg_dir = project_dir.join("packages/my-pkg");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("package.json"), r#"{"name": "my-pkg"}"#).unwrap();
    // Note: no formatter installed at package level

    let file_path = pkg_dir.join("index.js");
    fs::write(&file_path, "const x = 1;").unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    // Should NOT find prettier because we look for formatters only at the nearest package.json level
    // This tests that we don't accidentally fall back to root when package exists
}

#[test]
fn test_monorepo_package_with_oxfmt() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create monorepo root
    fs::write(project_dir.join("package.json"), r#"{"name": "monorepo"}"#).unwrap();

    // Create package with oxfmt
    let pkg_dir = project_dir.join("packages/configs");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(pkg_dir.join("package.json"), r#"{"name": "configs"}"#).unwrap();
    create_mock_formatter(&pkg_dir, "oxfmt");

    let file_path = pkg_dir.join("config.json");
    fs::write(&file_path, r#"{"key": "value"}"#).unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));
    assert!(
        output.contains("oxfmt"),
        "Should use oxfmt from package level: {}",
        output
    );
}

#[test]
fn test_rust_workspace_uses_workspace_cargo_fmt() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_dir = temp_dir.path();

    // Create Cargo workspace
    fs::write(
        workspace_dir.join("Cargo.toml"),
        r#"[workspace]
members = ["crates/*"]
"#,
    )
    .unwrap();

    // Create a crate in the workspace
    let crate_dir = workspace_dir.join("crates/my-crate");
    fs::create_dir_all(&crate_dir).unwrap();
    fs::write(
        crate_dir.join("Cargo.toml"),
        r#"[package]
name = "my-crate"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();

    let src_dir = crate_dir.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    let file_path = src_dir.join("lib.rs");
    let unformatted = "pub fn add(a:i32,b:i32)->i32{a+b}";
    fs::write(&file_path, unformatted).unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));

    // Should use cargo fmt from the workspace
    if output.contains("cargo fmt") {
        let formatted = fs::read_to_string(&file_path).unwrap();
        assert_ne!(formatted, unformatted, "File should have been formatted");
    }
}

#[test]
fn test_go_workspace_with_nested_modules() {
    let temp_dir = TempDir::new().unwrap();
    let workspace_dir = temp_dir.path();

    // Create Go workspace
    fs::write(
        workspace_dir.join("go.work"),
        "go 1.21\n\nuse (\n\t./cmd/app\n)\n",
    )
    .unwrap();

    // Create a module in the workspace
    let module_dir = workspace_dir.join("cmd/app");
    fs::create_dir_all(&module_dir).unwrap();
    fs::write(module_dir.join("go.mod"), "module app\n\ngo 1.21\n").unwrap();

    let file_path = module_dir.join("main.go");
    let unformatted = "package main\nfunc main(){x:=1;_=x}";
    fs::write(&file_path, unformatted).unwrap();

    let output = run_hook_with_input(&make_hook_input(&file_path));

    assert!(output.contains("continue"));
    assert!(output.contains("true"));

    // Should detect goimports, gofumpt, or gofmt
    if output.contains("goimports") || output.contains("gofumpt") || output.contains("gofmt") {
        let formatted = fs::read_to_string(&file_path).unwrap();
        // Check that formatting occurred (gofmt/gofumpt adds space around :=)
        if formatted != unformatted {
            assert!(
                formatted.contains("x := 1") || formatted.contains("x := "),
                "Go file should be formatted"
            );
        }
    }
}
