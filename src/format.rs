use std::path::Path;
use std::process::Command;

use crate::project::{
    find_cargo_root, find_go_root, find_java_root, find_node_root, find_project_root,
};

/// Result of a formatting operation
#[derive(Debug)]
#[allow(dead_code)]
pub struct FormatResult {
    pub formatted: bool,
    pub formatter: Option<String>,
    pub message: String,
}

impl FormatResult {
    pub fn success(formatter: &str) -> Self {
        Self {
            formatted: true,
            formatter: Some(formatter.to_string()),
            message: format!("Formatted with {}", formatter),
        }
    }

    pub fn no_formatter(language: &str) -> Self {
        Self {
            formatted: false,
            formatter: None,
            message: format!("No formatter found for {}", language),
        }
    }

    pub fn unsupported(ext: &str) -> Self {
        Self {
            formatted: false,
            formatter: None,
            message: format!("Unsupported file extension: {}", ext),
        }
    }

    pub fn error(formatter: &str, error: &str) -> Self {
        Self {
            formatted: false,
            formatter: Some(formatter.to_string()),
            message: format!("{} error: {}", formatter, error),
        }
    }
}

/// Format a file based on its extension
pub fn format_file(file_path: &Path) -> FormatResult {
    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => format_javascript(file_path),
        "rs" => format_rust(file_path),
        "py" | "pyi" => format_python(file_path),
        "java" => format_java(file_path),
        "go" => format_go(file_path),
        // oxfmt-supported formats
        "json" | "jsonc" | "json5" => format_with_oxfmt(file_path, "JSON"),
        "yaml" | "yml" => format_with_oxfmt(file_path, "YAML"),
        "toml" => format_with_oxfmt(file_path, "TOML"),
        "html" | "htm" => format_with_oxfmt(file_path, "HTML"),
        "vue" => format_with_oxfmt(file_path, "Vue"),
        "css" => format_with_oxfmt(file_path, "CSS"),
        "scss" => format_with_oxfmt(file_path, "SCSS"),
        "less" => format_with_oxfmt(file_path, "Less"),
        "md" | "markdown" => format_with_oxfmt(file_path, "Markdown"),
        "mdx" => format_with_oxfmt(file_path, "MDX"),
        "graphql" | "gql" => format_with_oxfmt(file_path, "GraphQL"),
        "hbs" | "handlebars" => format_with_oxfmt(file_path, "Handlebars"),
        _ => FormatResult::unsupported(ext),
    }
}

/// Format JavaScript/TypeScript files
fn format_javascript(file_path: &Path) -> FormatResult {
    let project_root = find_node_root(file_path);

    // Try biome first
    if let Some(ref root) = project_root {
        let biome_path = root.join("node_modules/.bin/biome");
        if biome_path.exists() {
            return run_formatter(
                "biome",
                &biome_path,
                &["format", "--write"],
                file_path,
                None,
            );
        }
    }

    // Try prettier
    if let Some(ref root) = project_root {
        let prettier_path = root.join("node_modules/.bin/prettier");
        if prettier_path.exists() {
            return run_formatter("prettier", &prettier_path, &["--write"], file_path, None);
        }
    }

    // Try dprint (global)
    if command_exists("dprint") {
        return run_formatter_cmd("dprint", &["fmt"], file_path, None);
    }

    FormatResult::no_formatter("JavaScript/TypeScript")
}

/// Format Rust files
fn format_rust(file_path: &Path) -> FormatResult {
    let project_root = find_cargo_root(file_path);

    // Try cargo fmt if in a Cargo project
    if let Some(ref root) = project_root {
        let result = Command::new("cargo")
            .args(["fmt", "--", file_path.to_str().unwrap_or("")])
            .current_dir(root)
            .output();

        match result {
            Ok(output) if output.status.success() => {
                return FormatResult::success("cargo fmt");
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return FormatResult::error("cargo fmt", &stderr);
            }
            Err(_) => {}
        }
    }

    // Fallback to rustfmt directly
    if command_exists("rustfmt") {
        return run_formatter_cmd("rustfmt", &[], file_path, None);
    }

    FormatResult::no_formatter("Rust")
}

/// Format Python files
fn format_python(file_path: &Path) -> FormatResult {
    // Try ruff format
    if command_exists("ruff") {
        let result = run_formatter_cmd("ruff", &["format"], file_path, None);
        if result.formatted {
            return result;
        }
    }

    // Try black
    if command_exists("black") {
        let result = run_formatter_cmd("black", &[], file_path, None);
        if result.formatted {
            return result;
        }
    }

    // Try autopep8
    if command_exists("autopep8") {
        let result = run_formatter_cmd("autopep8", &["--in-place"], file_path, None);
        if result.formatted {
            return result;
        }
    }

    // Try yapf
    if command_exists("yapf") {
        let result = run_formatter_cmd("yapf", &["-i"], file_path, None);
        if result.formatted {
            return result;
        }
    }

    FormatResult::no_formatter("Python")
}

/// Format Java files
fn format_java(file_path: &Path) -> FormatResult {
    let project_root = find_java_root(file_path);

    if let Some(ref root) = project_root {
        // Check for Maven with Spotless
        if root.join("pom.xml").exists() {
            let result = Command::new("mvn")
                .args([
                    "spotless:apply",
                    &format!("-DspotlessFiles={}", file_path.display()),
                ])
                .current_dir(root)
                .output();

            if let Ok(output) = result {
                if output.status.success() {
                    return FormatResult::success("spotless (Maven)");
                }
            }
        }

        // Check for Gradle with Spotless
        if root.join("build.gradle").exists() || root.join("build.gradle.kts").exists() {
            // Try gradlew first
            let gradlew = if cfg!(windows) {
                root.join("gradlew.bat")
            } else {
                root.join("gradlew")
            };

            let gradle_cmd = if gradlew.exists() {
                gradlew.to_str().unwrap_or("gradle").to_string()
            } else {
                "gradle".to_string()
            };

            let result = Command::new(&gradle_cmd)
                .args(["spotlessApply"])
                .current_dir(root)
                .output();

            if let Ok(output) = result {
                if output.status.success() {
                    return FormatResult::success("spotless (Gradle)");
                }
            }
        }
    }

    // Try google-java-format
    if command_exists("google-java-format") {
        return run_formatter_cmd("google-java-format", &["--replace"], file_path, None);
    }

    // Try palantir-java-format
    if command_exists("palantir-java-format") {
        return run_formatter_cmd("palantir-java-format", &["--replace"], file_path, None);
    }

    FormatResult::no_formatter("Java")
}

/// Format Go files
fn format_go(file_path: &Path) -> FormatResult {
    let project_root = find_go_root(file_path);
    let cwd = project_root.as_deref();

    // Best: goimports (imports) + gofumpt (strict formatting)
    if command_exists("goimports") && command_exists("gofumpt") {
        let result = run_formatter_cmd("goimports", &["-w"], file_path, cwd);
        if result.formatted {
            let result2 = run_formatter_cmd("gofumpt", &["-w"], file_path, cwd);
            if result2.formatted {
                return FormatResult::success("goimports + gofumpt");
            }
        }
    }

    // Try gofumpt alone (strict formatting, no import management)
    if command_exists("gofumpt") {
        let result = run_formatter_cmd("gofumpt", &["-w"], file_path, cwd);
        if result.formatted {
            return result;
        }
    }

    // Try goimports alone (imports + basic formatting)
    if command_exists("goimports") {
        let result = run_formatter_cmd("goimports", &["-w"], file_path, cwd);
        if result.formatted {
            return result;
        }
    }

    // Fallback to gofmt (always available with Go installation)
    if command_exists("gofmt") {
        return run_formatter_cmd("gofmt", &["-w"], file_path, cwd);
    }

    FormatResult::no_formatter("Go")
}

/// Format files using oxfmt (JSON, YAML, TOML, HTML, Vue, CSS, SCSS, Less, Markdown, MDX, GraphQL, Handlebars)
fn format_with_oxfmt(file_path: &Path, language: &str) -> FormatResult {
    let project_root = find_project_root(file_path);

    // Try project-local oxfmt first (node_modules/.bin/oxfmt)
    if let Some(ref root) = project_root {
        let oxfmt_path = root.join("node_modules/.bin/oxfmt");
        if oxfmt_path.exists() {
            return run_formatter("oxfmt", &oxfmt_path, &["--write"], file_path, None);
        }
    }

    // Fallback to global oxfmt
    if command_exists("oxfmt") {
        return run_formatter_cmd("oxfmt", &["--write"], file_path, None);
    }

    FormatResult::no_formatter(language)
}

/// Check if a command exists in PATH
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run a formatter command
fn run_formatter_cmd(
    name: &str,
    args: &[&str],
    file_path: &Path,
    cwd: Option<&Path>,
) -> FormatResult {
    let mut cmd = Command::new(name);
    cmd.args(args).arg(file_path);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    match cmd.output() {
        Ok(output) if output.status.success() => FormatResult::success(name),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            FormatResult::error(name, &stderr)
        }
        Err(e) => FormatResult::error(name, &e.to_string()),
    }
}

/// Run a formatter with a specific path
fn run_formatter(
    name: &str,
    formatter_path: &Path,
    args: &[&str],
    file_path: &Path,
    cwd: Option<&Path>,
) -> FormatResult {
    let mut cmd = Command::new(formatter_path);
    cmd.args(args).arg(file_path);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    match cmd.output() {
        Ok(output) if output.status.success() => FormatResult::success(name),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            FormatResult::error(name, &stderr)
        }
        Err(e) => FormatResult::error(name, &e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_result_success() {
        let result = FormatResult::success("prettier");
        assert!(result.formatted);
        assert_eq!(result.formatter, Some("prettier".to_string()));
    }

    #[test]
    fn test_format_result_no_formatter() {
        let result = FormatResult::no_formatter("JavaScript");
        assert!(!result.formatted);
        assert!(result.formatter.is_none());
    }

    #[test]
    fn test_format_result_unsupported() {
        let result = FormatResult::unsupported("xyz");
        assert!(!result.formatted);
        assert!(result.message.contains("xyz"));
    }

    #[test]
    fn test_unsupported_extension() {
        let result = format_file(Path::new("/path/to/file.unknown"));
        assert!(!result.formatted);
        assert!(result.message.contains("Unsupported"));
    }
}
