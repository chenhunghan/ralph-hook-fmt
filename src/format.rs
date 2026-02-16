use std::path::Path;
use std::process::Command;

use crate::project::{
    find_cargo_root, find_go_root, find_java_root, find_node_root, find_project_root,
    find_python_root,
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
pub fn format_file(file_path: &Path, project_only: bool) -> FormatResult {
    // Skip package.json - formatting can reorder keys and break package managers
    if let Some(name) = file_path.file_name().and_then(|n| n.to_str()) {
        if name == "package.json" {
            return FormatResult {
                formatted: false,
                formatter: None,
                message: "Skipped package.json".to_string(),
            };
        }
    }

    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

    match ext {
        "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => format_javascript(file_path, project_only),
        "rs" => format_rust(file_path, project_only),
        "py" | "pyi" => format_python(file_path, project_only),
        "java" => format_java(file_path, project_only),
        "go" => format_go(file_path, project_only),
        // oxfmt-supported formats
        "json" | "jsonc" | "json5" => format_with_oxfmt(file_path, "JSON", project_only),
        "yaml" | "yml" => format_with_oxfmt(file_path, "YAML", project_only),
        "toml" => format_with_oxfmt(file_path, "TOML", project_only),
        "html" | "htm" => format_with_oxfmt(file_path, "HTML", project_only),
        "vue" => format_with_oxfmt(file_path, "Vue", project_only),
        "css" => format_with_oxfmt(file_path, "CSS", project_only),
        "scss" => format_with_oxfmt(file_path, "SCSS", project_only),
        "less" => format_with_oxfmt(file_path, "Less", project_only),
        "md" | "markdown" => format_with_oxfmt(file_path, "Markdown", project_only),
        "mdx" => format_with_oxfmt(file_path, "MDX", project_only),
        "graphql" | "gql" => format_with_oxfmt(file_path, "GraphQL", project_only),
        "hbs" | "handlebars" => format_with_oxfmt(file_path, "Handlebars", project_only),
        _ => FormatResult::unsupported(ext),
    }
}

/// Format JavaScript/TypeScript files
fn format_javascript(file_path: &Path, project_only: bool) -> FormatResult {
    let project_root = find_node_root(file_path);

    // Try local formatters first (in priority order)
    if let Some(ref root) = project_root {
        // Try oxfmt first (fastest)
        let oxfmt_path = root.join("node_modules/.bin/oxfmt");
        if oxfmt_path.exists() {
            return run_formatter("oxfmt", &oxfmt_path, &["--write"], file_path, None);
        }

        // Try biome
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

        // Try prettier
        let prettier_path = root.join("node_modules/.bin/prettier");
        if prettier_path.exists() {
            return run_formatter("prettier", &prettier_path, &["--write"], file_path, None);
        }
    }

    if !project_only {
        // Fall back to global formatters
        if command_exists("oxfmt") {
            return run_formatter_cmd("oxfmt", &["--write"], file_path, None);
        }

        if command_exists("dprint") {
            return run_formatter_cmd("dprint", &["fmt"], file_path, None);
        }
    }

    FormatResult::no_formatter("JavaScript/TypeScript")
}

/// Format Rust files
fn format_rust(file_path: &Path, project_only: bool) -> FormatResult {
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

    if !project_only {
        // Fallback to rustfmt directly
        if command_exists("rustfmt") {
            return run_formatter_cmd("rustfmt", &[], file_path, None);
        }
    }

    FormatResult::no_formatter("Rust")
}

/// Format Python files
fn format_python(file_path: &Path, project_only: bool) -> FormatResult {
    let formatters = ["ruff", "black", "autopep8", "yapf"];
    let formatter_args: &[&[&str]] = &[&["format"], &[], &["--in-place"], &["-i"]];

    if project_only {
        // In project-only mode, only check for formatters in local venv
        if let Some(ref root) = find_python_root(file_path) {
            let venv_dirs = [".venv", "venv"];
            for (i, name) in formatters.iter().enumerate() {
                for venv_dir in &venv_dirs {
                    let formatter_path = root.join(venv_dir).join("bin").join(name);
                    if formatter_path.exists() {
                        return run_formatter(
                            name,
                            &formatter_path,
                            formatter_args[i],
                            file_path,
                            None,
                        );
                    }
                }
            }
        }
    } else {
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
    }

    FormatResult::no_formatter("Python")
}

/// Format Java files
fn format_java(file_path: &Path, project_only: bool) -> FormatResult {
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

    if !project_only {
        // Try google-java-format
        if command_exists("google-java-format") {
            return run_formatter_cmd("google-java-format", &["--replace"], file_path, None);
        }

        // Try palantir-java-format
        if command_exists("palantir-java-format") {
            return run_formatter_cmd("palantir-java-format", &["--replace"], file_path, None);
        }
    }

    FormatResult::no_formatter("Java")
}

/// Format Go files
fn format_go(file_path: &Path, project_only: bool) -> FormatResult {
    let project_root = find_go_root(file_path);

    if project_only && project_root.is_none() {
        return FormatResult::no_formatter("Go");
    }

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
fn format_with_oxfmt(file_path: &Path, language: &str, project_only: bool) -> FormatResult {
    let project_root = find_project_root(file_path);

    // Try project-local oxfmt first (node_modules/.bin/oxfmt)
    if let Some(ref root) = project_root {
        let oxfmt_path = root.join("node_modules/.bin/oxfmt");
        if oxfmt_path.exists() {
            return run_formatter("oxfmt", &oxfmt_path, &["--write"], file_path, None);
        }
    }

    if !project_only {
        // Fallback to global oxfmt
        if command_exists("oxfmt") {
            return run_formatter_cmd("oxfmt", &["--write"], file_path, None);
        }
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
        let result = format_file(Path::new("/path/to/file.unknown"), false);
        assert!(!result.formatted);
        assert!(result.message.contains("Unsupported"));
    }

    #[test]
    fn test_skip_package_json() {
        let result = format_file(Path::new("/path/to/package.json"), false);
        assert!(!result.formatted);
        assert!(result.message.contains("Skipped package.json"));
    }
}
