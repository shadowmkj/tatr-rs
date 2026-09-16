use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tatr_rs::DIRNAME;
use tatr_rs::generate_datetime_id;
use tatr_rs::task::Task;

const ACTIONS: &[&str] = &[
    "Implement",
    "Fix bug in",
    "Refactor",
    "Optimize",
    "Add unit tests for",
    "Improve error handling in",
    "Document",
    "Benchmark",
];

const COMPONENTS: &[&str] = &[
    "lexer token streaming",
    "parser precedence handling",
    "stack machine VM execution",
    "query compiler Shunting-Yard logic",
    "CLI output formatting",
    "task metadata serialization",
    "tag indexing and caching",
    "issue search filter",
];

const AVAILABLE_TAGS: &[&str] = &[
    "parser", "lexer", "vm", "compiler", "cli", "bug", "feature", "perf", "docs", "refactor",
];

const DESCRIPTIONS: &[&str] = &[
    "Investigate edge cases and ensure test coverage for all code paths.",
    "Needs cleanup and better error messages when encountering invalid syntax.",
    "Profile execution and minimize memory allocations where possible.",
    "Follows the specification defined in the project documentation.",
    "Add regression tests and verify behavior with various input combinations.",
];

/// Simple deterministic pseudo-random number generator (Linear Congruential Generator)
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn choose<'a, T>(&mut self, slice: &'a [T]) -> &'a T {
        let idx = (self.next_u32() as usize) % slice.len();
        &slice[idx]
    }

    fn range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            min
        } else {
            min + (self.next_u32() % (max - min + 1))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(10);

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(123456789);

    let mut rng = SimpleRng::new(seed);

    println!("Generating {count} test tasks into '{DIRNAME}/'...");

    for i in 0..count {
        let action = rng.choose(ACTIONS);
        let component = rng.choose(COMPONENTS);
        let title = format!("{action} {component}");

        // Random priority between 10 and 100 in steps of 10
        let priority = (rng.range(1, 10) * 10) as u8;

        // Pick 1 to 3 unique tags
        let tag_count = rng.range(1, 3) as usize;
        let mut selected_tags: Vec<String> = Vec::new();
        for _ in 0..tag_count {
            let tag = (*rng.choose(AVAILABLE_TAGS)).to_string();
            if !selected_tags.contains(&tag) {
                selected_tags.push(tag);
            }
        }

        let body = format!(
            "{}\n\nGenerated automatically for testing query filters.",
            rng.choose(DESCRIPTIONS)
        );

        if i > 0 {
            // Sleep for 1 second so generate_datetime_id() produces a new distinct timestamp
            std::thread::sleep(std::time::Duration::from_millis(1050));
        }

        let id = generate_datetime_id();
        let task = Task::new(&id, &title, priority, selected_tags, &body);

        match task.save() {
            Ok(_) => {
                println!(
                    "  [+] Created: {DIRNAME}/{id}/TASK.md (Priority: {priority}, Tags: {:?})",
                    task.to_markdown()
                        .lines()
                        .find(|l| l.starts_with("- TAGS:"))
                        .unwrap_or("")
                );
            }
            Err(err) => {
                eprintln!("  [-] Failed to save task {id}: {err}");
            }
        }
    }

    println!("\nDone! You can now test queries like:");
    println!("  cargo run -- ls -q \"parser and not bug\"");
    println!("  cargo run -- ls -q \"feature or perf\"");
}
