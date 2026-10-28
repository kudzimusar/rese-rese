use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    process,
};

fn root() -> PathBuf {
    let mut path = env::current_dir().expect("working directory");
    loop {
        if path.join("docs/AUTHORITY.md").exists() && path.join("Cargo.toml").exists() {
            return path;
        }
        assert!(path.pop(), "run inside rese-rese repository");
    }
}

fn markdown_files(dir: &Path, into: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for item in fs::read_dir(dir)? {
        let path = item?.path();
        if path.is_dir() && path.file_name().and_then(|x| x.to_str()) != Some("target") {
            markdown_files(&path, into)?;
        } else if path.extension().and_then(|x| x.to_str()) == Some("md") {
            into.push(path);
        }
    }
    Ok(())
}

fn local_links(doc: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut text = doc;
    while let Some(offset) = text.find("](") {
        text = &text[offset + 2..];
        if let Some(end) = text.find(')') {
            let link = text[..end].split_whitespace().next().unwrap_or("");
            if !link.is_empty()
                && !link.starts_with('#')
                && !link.starts_with("http:")
                && !link.starts_with("https:")
                && !link.starts_with("mailto:")
                && !link.starts_with("data:")
                && !link.starts_with('/') {
                links.push(link.split('#').next().unwrap_or("").to_string());
            }
            text = &text[end + 1..];
        } else {
            break;
        }
    }
    links
}

fn check() -> Result<(), String> {
    let home = root();
    let mandatory = [
        "README.md",
        "AGENTS.md",
        "docs/AUTHORITY.md",
        "docs/roadmap/tasks.csv",
        "docs/source/RR-MASTER-01.original.md",
        "docs/source/RR-AI-CORE-01.original.md",
        "docs/source/RR-GEO-AI-01.original.md",
        "docs/decisions/ADR-0002-rust-trailbase.md",
    ];
    for path in mandatory {
        if !home.join(path).exists() {
            return Err(format!("Missing mandatory file: {path}"));
        }
    }
    let mut docs = Vec::new();
    markdown_files(&home, &mut docs).map_err(|e| e.to_string())?;
    let mut issues = Vec::new();
    for path in &docs {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        for link in local_links(&content) {
            if !path.parent().expect("parent").join(&link).exists() {
                issues.push(format!("Broken local link: {} -> {link}", path.display()));
            }
        }
    }
    let registry = fs::read_to_string(home.join("docs/roadmap/tasks.csv"))
        .map_err(|e| e.to_string())?;
    let mut ids = HashSet::new();
    let mut count = 0;
    for row in registry.lines().skip(1) {
        let id = row.split(',').next().unwrap_or("").trim_matches('"');
        if !id.starts_with("RR-") {
            issues.push(format!("Malformed work-package ID: {id}"));
        }
        if !ids.insert(id.to_string()) {
            issues.push(format!("Duplicate work-package ID: {id}"));
        }
        count += 1;
    }
    if count != 57 {
        issues.push(format!("Expected 57 tasks; found {count}"));
    }
    let readme = fs::read_to_string(home.join("README.md")).map_err(|e| e.to_string())?;
    for term in ["Rust", "TrailBase", "Skills", "Communities", "GEO", "free"] {
        if !readme.contains(term) {
            issues.push(format!("README missing required concept: {term}"));
        }
    }
    if issues.is_empty() {
        println!("PASS: {count} tasks; {} Markdown files; links and authority present", docs.len());
        Ok(())
    } else {
        Err(issues.join("\n"))
    }
}

fn main() {
    if let Err(error) = check() {
        eprintln!("Documentation check failed:\n{error}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authority_links_and_tasks() { check().expect("documentation integrity failure"); }
}
