use std::path::{Path, PathBuf};

/// Find the project root by looking for common project markers
pub fn find_project_root(file_path: &Path) -> Option<PathBuf> {
    let mut current = file_path.parent()?;

    loop {
        // Check for common project root markers
        let markers = [
            "Cargo.toml",
            "package.json",
            "pyproject.toml",
            "setup.py",
            "pom.xml",
            "build.gradle",
            "build.gradle.kts",
            "go.mod",
            ".git",
        ];

        for marker in markers {
            if current.join(marker).exists() {
                return Some(current.to_path_buf());
            }
        }

        // Move up to parent directory
        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Find the nearest Cargo.toml for Rust projects
pub fn find_cargo_root(file_path: &Path) -> Option<PathBuf> {
    let mut current = file_path.parent()?;

    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            return Some(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Find the nearest package.json for Node.js projects
pub fn find_node_root(file_path: &Path) -> Option<PathBuf> {
    let mut current = file_path.parent()?;

    loop {
        let package_json = current.join("package.json");
        if package_json.exists() {
            return Some(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Find the nearest Python project root (pyproject.toml or setup.py)
#[allow(dead_code)]
pub fn find_python_root(file_path: &Path) -> Option<PathBuf> {
    let mut current = file_path.parent()?;

    loop {
        if current.join("pyproject.toml").exists() || current.join("setup.py").exists() {
            return Some(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Find the nearest Maven/Gradle project root
pub fn find_java_root(file_path: &Path) -> Option<PathBuf> {
    let mut current = file_path.parent()?;

    loop {
        let markers = ["pom.xml", "build.gradle", "build.gradle.kts"];
        for marker in markers {
            if current.join(marker).exists() {
                return Some(current.to_path_buf());
            }
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

/// Find the nearest go.mod for Go projects
pub fn find_go_root(file_path: &Path) -> Option<PathBuf> {
    let mut current = file_path.parent()?;

    loop {
        let go_mod = current.join("go.mod");
        if go_mod.exists() {
            return Some(current.to_path_buf());
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_cargo_root() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("my_project");
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(project_dir.join("Cargo.toml"), "[package]").unwrap();

        let file_path = src_dir.join("main.rs");
        let root = find_cargo_root(&file_path).unwrap();
        assert_eq!(root, project_dir);
    }

    #[test]
    fn test_find_node_root() {
        let temp_dir = TempDir::new().unwrap();
        let project_dir = temp_dir.path().join("my_project");
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(project_dir.join("package.json"), "{}").unwrap();

        let file_path = src_dir.join("index.js");
        let root = find_node_root(&file_path).unwrap();
        assert_eq!(root, project_dir);
    }
}
