use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

use crate::DIRNAME;

#[derive(Debug)]
pub enum FindTaskError {
    Ambiguous(Vec<String>),
    Io(io::Error),
}

impl From<io::Error> for FindTaskError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl std::fmt::Display for FindTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FindTaskError::Ambiguous(matches) => {
                write!(
                    f,
                    "Ambiguous task ID query. Multiple matches found: {}",
                    matches.join(", ")
                )
            }
            FindTaskError::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for FindTaskError {}

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
        self.save_to_dir(Path::new(DIRNAME))
    }

    pub fn save_to_dir(&self, base_dir: &Path) -> io::Result<()> {
        let dir_path: PathBuf = base_dir.join(&self.id);
        let file_path: PathBuf = dir_path.join("TASK.md");
        fs::create_dir_all(&dir_path)?;
        fs::write(file_path, self.to_markdown())?;

        Ok(())
    }

    /// Closes the task and persists the status change to disk in default issues directory.
    pub fn close(&mut self) -> io::Result<()> {
        self.close_in_dir(Path::new(DIRNAME))
    }

    /// Closes the task and persists the status change to disk in specified base directory.
    pub fn close_in_dir(&mut self, base_dir: &Path) -> io::Result<()> {
        self.status = "CLOSED".to_string();
        self.save_to_dir(base_dir)
    }

    /// Reopens the task and persists the status change to disk in default issues directory.
    pub fn reopen(&mut self) -> io::Result<()> {
        self.reopen_in_dir(Path::new(DIRNAME))
    }

    /// Reopens the task and persists the status change to disk in specified base directory.
    pub fn reopen_in_dir(&mut self, base_dir: &Path) -> io::Result<()> {
        self.status = "OPEN".to_string();
        self.save_to_dir(base_dir)
    }

    /// Finds a task in the default issues directory matching an exact or partial ID.
    pub fn find(id_query: &str) -> Result<Option<Self>, FindTaskError> {
        Self::find_in_dir(Path::new(DIRNAME), id_query)
    }

    /// Finds a task in the specified directory matching an exact or partial ID.
    pub fn find_in_dir(base_dir: &Path, id_query: &str) -> Result<Option<Self>, FindTaskError> {
        if !base_dir.exists() {
            return Ok(None);
        }

        // 1. Try exact match first
        let exact_path = base_dir.join(id_query);
        if exact_path.is_dir()
            && let Some(task) = Self::from_dir(&exact_path)?
        {
            return Ok(Some(task));
        }

        // 2. Scan for partial/prefix match
        let mut matches = Vec::new();
        for entry in fs::read_dir(base_dir)?.flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Some(name) = path.file_name().and_then(|n| n.to_str())
                && name.contains(id_query)
            {
                matches.push((name.to_string(), path));
            }
        }

        if matches.is_empty() {
            Ok(None)
        } else if matches.len() == 1 {
            let (_, path) = matches.remove(0);
            Ok(Self::from_dir(&path)?)
        } else {
            Err(FindTaskError::Ambiguous(
                matches.into_iter().map(|(name, _)| name).collect(),
            ))
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_task_close_and_reopen() {
        let tmp_dir = std::env::temp_dir().join("tatr_test_close_reopen");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir).unwrap();

        let mut task = Task::new(
            "20261011-000001",
            "Test Close/Reopen Task",
            80,
            vec!["test"],
            "Testing status transitions",
        );
        assert_eq!(task.status, "OPEN");

        // Save initially
        task.save_to_dir(&tmp_dir).unwrap();

        // Close the task
        task.close_in_dir(&tmp_dir).unwrap();
        assert_eq!(task.status, "CLOSED");

        // Verify persisted state
        let reloaded = Task::find_in_dir(&tmp_dir, "20261011-000001")
            .unwrap()
            .unwrap();
        assert_eq!(reloaded.status, "CLOSED");

        // Reopen the task
        task.reopen_in_dir(&tmp_dir).unwrap();
        assert_eq!(task.status, "OPEN");

        // Verify persisted state again
        let reloaded2 = Task::find_in_dir(&tmp_dir, "20261011-000001")
            .unwrap()
            .unwrap();
        assert_eq!(reloaded2.status, "OPEN");

        let _ = fs::remove_dir_all(&tmp_dir);
    }

    #[test]
    fn test_find_exact_and_prefix() {
        let tmp_dir = std::env::temp_dir().join("tatr_test_find_prefix");
        let _ = fs::remove_dir_all(&tmp_dir);
        fs::create_dir_all(&tmp_dir).unwrap();

        let task1 = Task::new("20261011-111111", "Task 1", 50, vec!["cli"], "");
        let task2 = Task::new("20261011-222222", "Task 2", 70, vec!["vm"], "");

        task1.save_to_dir(&tmp_dir).unwrap();
        task2.save_to_dir(&tmp_dir).unwrap();

        let found1 = Task::find_in_dir(&tmp_dir, "111111").unwrap().unwrap();
        assert_eq!(found1.title, "Task 1");

        let found2 = Task::find_in_dir(&tmp_dir, "20261011-222222")
            .unwrap()
            .unwrap();
        assert_eq!(found2.title, "Task 2");

        match Task::find_in_dir(&tmp_dir, "20261011") {
            Err(FindTaskError::Ambiguous(matches)) => {
                assert_eq!(matches.len(), 2);
            }
            _ => panic!("Expected Ambiguous error"),
        }

        let not_found = Task::find_in_dir(&tmp_dir, "999999").unwrap();
        assert!(not_found.is_none());

        let _ = fs::remove_dir_all(&tmp_dir);
    }
}
