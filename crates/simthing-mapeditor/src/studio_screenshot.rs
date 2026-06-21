//! STUDIO-PERFORMANCE-TELEMETRY-WINDOW-0 — screenshot filename allocation (presentation only).

use std::path::Path;

pub const SCREENSHOT_FILENAME_PREFIX: &str = "screenshot_";
pub const SCREENSHOT_FILENAME_SUFFIX: &str = ".png";
pub const SCREENSHOT_FILENAME_DIGITS: usize = 5;

/// Parse `screenshot_00042.png` → `Some(42)`; ignores unrelated files.
pub fn parse_screenshot_index(filename: &str) -> Option<u32> {
    let stem = filename.strip_prefix(SCREENSHOT_FILENAME_PREFIX)?;
    let index_str = stem.strip_suffix(SCREENSHOT_FILENAME_SUFFIX)?;
    if index_str.len() != SCREENSHOT_FILENAME_DIGITS
        || !index_str.chars().all(|c| c.is_ascii_digit())
    {
        return None;
    }
    index_str.parse().ok()
}

/// Next available `screenshot_{index:05}.png` in `directory` (monotonic by parsed index).
pub fn next_screenshot_filename(directory: &Path) -> Option<String> {
    let mut max_index: Option<u32> = None;
    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if let Some(index) = parse_screenshot_index(&name) {
                max_index = Some(max_index.map_or(index, |current| current.max(index)));
            }
        }
    }
    let next = max_index.map(|index| index.saturating_add(1)).unwrap_or(0);
    Some(format!(
        "{SCREENSHOT_FILENAME_PREFIX}{next:0width$}{SCREENSHOT_FILENAME_SUFFIX}",
        width = SCREENSHOT_FILENAME_DIGITS
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn next_screenshot_filename_starts_at_zero() {
        let dir = tempdir().expect("tempdir");
        assert_eq!(
            next_screenshot_filename(dir.path()),
            Some("screenshot_00000.png".to_string())
        );
    }

    #[test]
    fn next_screenshot_filename_skips_existing() {
        let dir = tempdir().expect("tempdir");
        fs::write(dir.path().join("screenshot_00000.png"), b"x").expect("write");
        assert_eq!(
            next_screenshot_filename(dir.path()),
            Some("screenshot_00001.png".to_string())
        );
        fs::write(dir.path().join("screenshot_00002.png"), b"x").expect("write");
        assert_eq!(
            next_screenshot_filename(dir.path()),
            Some("screenshot_00003.png".to_string())
        );
    }

    #[test]
    fn next_screenshot_filename_ignores_unrelated_files() {
        let dir = tempdir().expect("tempdir");
        fs::write(dir.path().join("notes.txt"), b"x").expect("write");
        fs::write(dir.path().join("screenshot.png"), b"x").expect("write");
        fs::write(dir.path().join("screenshot_42.png"), b"x").expect("write");
        assert_eq!(
            next_screenshot_filename(dir.path()),
            Some("screenshot_00000.png".to_string())
        );
    }
}
