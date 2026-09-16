use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

use crate::DIRNAME;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: String,
    pub tags: Vec<String>,
    pub priority: u8, // Usually use 0 - 100
    pub body: String,
}

impl Task {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        priority: u8,
        tags: impl IntoIterator<Item = impl Into<String>>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            status: "OPEN".to_string(),
            tags: tags.into_iter().map(Into::into).collect(),
            priority,
            body: body.into(),
        }
    }


    /// Formats the task into the Tatr Markdown specification.
    pub fn to_markdown(&self) -> String {
        let tags_str = self.tags.join(",");
        let body = if self.body.is_empty() {
            String::new()
        } else {
            format!("{}\n", self.body.trim())
        };

        format!(
            "# {}\n\n- STATUS: {}\n- PRIORITY: {}\n- TAGS: {}\n\n{}",
            self.title, self.status, self.priority, tags_str, body
        )
    }

    pub fn save(&self) -> io::Result<()> {
        let dir_path: PathBuf = Path::new(DIRNAME).join(&self.id);
        let file_path: PathBuf = dir_path.join("TASK.md");
        fs::create_dir_all(&dir_path)?;
        fs::write(file_path, self.to_markdown())?;

        Ok(())
    }

    /// Parses a Task from its folder ID and TASK.md content.
    pub fn from_markdown(id: impl Into<String>, content: &str) -> Option<Self> {
        let mut lines = content.lines();

        // 1. First line: # Title
        let title_line = lines.next()?.trim();
        let title = title_line.strip_prefix('#')?.trim().to_string();

        let mut status = "OPEN".to_string();
        let mut priority = 100;
        let mut tags = Vec::new();
        let mut body_lines = Vec::new();
        let mut in_body = false;

        for line in lines {
            let trimmed = line.trim();
            if !in_body {
                if trimmed.is_empty() {
                    continue;
                }
                if let Some(p) = trimmed.strip_prefix("- PRIORITY:") {
                    priority = p.trim().parse().unwrap_or(100);
                } else if let Some(t) = trimmed.strip_prefix("- TAGS:") {
                    tags = t
                        .split(',')
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(String::from)
                        .collect();
                } else if let Some(s) = trimmed.strip_prefix("- STATUS:") {
                    status = s.trim().to_string();
                } else {
                    in_body = true;
                    body_lines.push(line);
                }
            } else {
                body_lines.push(line);
            }
        }

        Some(Self {
            id: id.into(),
            title,
            status,
            tags,
            priority,
            body: body_lines.join("\n").trim().to_string(),
        })
    }


    /// Reads and parses `<dir>/TASK.md`, taking the folder name as the task ID.
    pub fn from_dir(dir_path: &Path) -> io::Result<Option<Self>> {
        let file_path = dir_path.join("TASK.md");
        if !file_path.is_file() {
            return Ok(None);
        }
        let content = fs::read_to_string(file_path)?;
        let id = dir_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();

        Ok(Self::from_markdown(id, &content))
    }

    /// Returns the task's tags as a HashSet for evaluation with Vm.
    pub fn tag_set(&self) -> HashSet<String> {
        self.tags.iter().cloned().collect()
    }
}

