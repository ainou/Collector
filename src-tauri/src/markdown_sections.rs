#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InsertPosition {
    Top,
    Bottom,
}

/// Insert `entry` into `content` under the specified markdown heading.
///
/// * `content` — existing markdown text
/// * `entry` — text to insert
/// * `target_heading` — exact heading to find, e.g. `"## Log"`
/// * `position` — where within the section to insert
/// * `create_heading_if_missing` — if true, append heading+entry at end when heading not found
///
/// Returns the modified content. Never panics.
pub fn insert_into_markdown_section(
    content: &str,
    entry: &str,
    target_heading: &str,
    position: InsertPosition,
    create_heading_if_missing: bool,
) -> String {
    let entry = entry.trim_end_matches('\n');
    let target = target_heading.trim();

    // Empty target = legacy append-to-end fallback
    if target.is_empty() {
        let mut result = content.to_string();
        if !result.is_empty() && !result.ends_with('\n') {
            result.push('\n');
        }
        result.push_str(entry);
        result.push('\n');
        return result;
    }

    let target_level = target.chars().take_while(|&c| c == '#').count();
    let heading_text = target[target_level..].trim();

    let lines: Vec<&str> = content.lines().collect();

    // Find first exact matching heading
    let heading_idx = lines.iter().position(|line| {
        let trimmed = line.trim();
        let level = trimmed.chars().take_while(|&c| c == '#').count();
        level > 0 && level == target_level && trimmed[level..].trim() == heading_text
    });

    match heading_idx {
        Some(idx) => {
            // Find end of this section: next heading of same or higher level (<= target_level)
            let section_end = lines[idx + 1..]
                .iter()
                .position(|line| {
                    let trimmed = line.trim();
                    let level = trimmed.chars().take_while(|&c| c == '#').count();
                    level > 0 && level <= target_level
                })
                .map(|end| idx + 1 + end)
                .unwrap_or(lines.len());

            let mut result = String::new();

            match position {
                InsertPosition::Top => {
                    // Heading line
                    for line in &lines[..=idx] {
                        result.push_str(line);
                        result.push('\n');
                    }
                    // Entry right after heading
                    result.push_str(entry);
                    result.push('\n');
                    // Existing section content
                    for line in &lines[idx + 1..section_end] {
                        result.push_str(line);
                        result.push('\n');
                    }
                }
                InsertPosition::Bottom => {
                    // Heading + existing section content
                    for line in &lines[..section_end] {
                        result.push_str(line);
                        result.push('\n');
                    }
                    // Entry at bottom of section
                    result.push_str(entry);
                    result.push('\n');
                }
            }

            // Lines after section
            for line in &lines[section_end..] {
                result.push_str(line);
                result.push('\n');
            }

            result
        }
        None => {
            if create_heading_if_missing {
                let mut result = content.to_string();
                // Ensure content ends with newline
                if !result.is_empty() && !result.ends_with('\n') {
                    result.push('\n');
                }
                // Blank line separator if there's existing content
                if !result.is_empty() && !result.ends_with("\n\n") {
                    result.push('\n');
                }
                result.push_str(target_heading);
                result.push('\n');
                result.push_str(entry);
                result.push('\n');
                result
            } else {
                // Fallback: append to end
                let mut result = content.to_string();
                if !result.is_empty() && !result.ends_with('\n') {
                    result.push('\n');
                }
                result.push_str(entry);
                result.push('\n');
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_bottom_into_log_section() {
        let content = "# Daily\n\n## Log\n\n- entry 1\n\n## Habits\n\n- habit 1\n";
        let result = insert_into_markdown_section(
            content,
            "- entry 2",
            "## Log",
            InsertPosition::Bottom,
            false,
        );
        assert!(result.contains("- entry 1"));
        assert!(result.contains("- entry 2"));
        // entry 2 must appear after entry 1 but before ## Habits
        let log_pos = result.find("## Log").unwrap();
        let entry1 = result.find("- entry 1").unwrap();
        let entry2 = result.find("- entry 2").unwrap();
        let habits_pos = result.find("## Habits").unwrap();
        assert!(log_pos < entry1);
        assert!(entry1 < entry2);
        assert!(entry2 < habits_pos);
    }

    #[test]
    fn test_insert_top_into_log_section() {
        let content = "# Daily\n\n## Log\n\n- entry 1\n\n## Habits\n";
        let result = insert_into_markdown_section(
            content,
            "- new entry",
            "## Log",
            InsertPosition::Top,
            false,
        );
        assert!(result.contains("- new entry"));
        assert!(result.contains("- entry 1"));
        // new entry must be first under ## Log
        let log_pos = result.find("## Log").unwrap();
        let new_entry_pos = result.find("- new entry").unwrap();
        let entry1_pos = result.find("- entry 1").unwrap();
        let habits_pos = result.find("## Habits").unwrap();
        assert!(log_pos < new_entry_pos);
        assert!(new_entry_pos < entry1_pos);
        assert!(entry1_pos < habits_pos);
    }

    #[test]
    fn test_insert_bottom_respects_next_heading_same_level() {
        let content = "## Log\n- a\n\n## Log\n- b\n";
        let result = insert_into_markdown_section(
            content,
            "- inserted",
            "## Log",
            InsertPosition::Bottom,
            false,
        );
        // Only first matching section gets the entry
        let log_count = result.matches("- inserted").count();
        assert_eq!(log_count, 1);
        // Inserted after -a, before second ## Log
        let a_pos = result.find("- a").unwrap();
        let inserted_pos = result.find("- inserted").unwrap();
        let b_pos = result.find("- b").unwrap();
        assert!(a_pos < inserted_pos);
        assert!(inserted_pos < b_pos);
    }

    #[test]
    fn test_insert_bottom_before_higher_level_heading() {
        // Entry inside a ### sub-section stays before a ## heading
        let content = "## Section A\n\n### Sub\n- item\n\n## Section B\n";
        let result = insert_into_markdown_section(
            content,
            "- new sub item",
            "### Sub",
            InsertPosition::Bottom,
            false,
        );
        assert!(result.contains("- item"));
        assert!(result.contains("- new sub item"));
        let sub_pos = result.find("### Sub").unwrap();
        let new_pos = result.find("- new sub item").unwrap();
        let section_b_pos = result.find("## Section B").unwrap();
        assert!(sub_pos < new_pos);
        assert!(new_pos < section_b_pos);
    }

    #[test]
    fn test_heading_missing_create_true() {
        let content = "# Note\n\nSome content.\n";
        let result = insert_into_markdown_section(
            content,
            "- captured item",
            "## Log",
            InsertPosition::Bottom,
            true,
        );
        assert!(result.contains("## Log"));
        assert!(result.contains("- captured item"));
        // Heading must be at the end
        let heading_pos = result.find("## Log").unwrap();
        let content_pos = result.find("Some content").unwrap();
        assert!(content_pos < heading_pos);
    }

    #[test]
    fn test_heading_missing_create_false() {
        let content = "# Note\n\nSome content.\n";
        let result =
            insert_into_markdown_section(content, "- orphan", "## Log", InsertPosition::Top, false);
        // No heading created — entry appended at end
        assert!(!result.contains("## Log"));
        assert!(result.contains("- orphan"));
        let content_pos = result.find("Some content").unwrap();
        let orphan_pos = result.find("- orphan").unwrap();
        assert!(content_pos < orphan_pos);
    }

    #[test]
    fn test_empty_content() {
        let result =
            insert_into_markdown_section("", "- first entry", "## Log", InsertPosition::Top, true);
        assert_eq!(result, "## Log\n- first entry\n");
    }

    #[test]
    fn test_empty_target_heading_fallback() {
        let content = "existing line\n";
        let result =
            insert_into_markdown_section(content, "appended", "", InsertPosition::Bottom, false);
        assert_eq!(result, "existing line\nappended\n");
    }

    #[test]
    fn test_preserves_exact_heading_level() {
        // ### Log should NOT match ## Log (different level)
        // But ### is a sub-heading inside ##, so bottom of ## goes after ### content.
        let content = "## Log\n- level 2\n\n### Log\n- level 3\n";
        let result = insert_into_markdown_section(
            content,
            "- new l2",
            "## Log",
            InsertPosition::Bottom,
            false,
        );
        // The entry goes in first ## Log section (bottom) — after all sub-content including ###
        assert!(result.contains("- new l2"));
        assert!(result.contains("- level 2"));
        assert!(result.contains("- level 3"));
        let l2_heading = result.find("## Log").unwrap();
        let new_l2 = result.find("- new l2").unwrap();
        let l3_heading = result.find("### Log").unwrap();
        // both level-2 and level-3 content come before the new entry
        assert!(l3_heading < new_l2);
        assert!(l2_heading < new_l2);
    }

    #[test]
    fn test_heading_with_trailing_spaces_matches() {
        let content = "## Log   \n- entry\n";
        let result =
            insert_into_markdown_section(content, "- new", "## Log", InsertPosition::Bottom, false);
        assert!(result.contains("- entry"));
        assert!(result.contains("- new"));
        let entry_pos = result.find("- entry").unwrap();
        let new_pos = result.find("- new").unwrap();
        assert!(entry_pos < new_pos);
    }

    #[test]
    fn test_target_heading_trimmed() {
        let content = "## Log\n- entry\n";
        let result = insert_into_markdown_section(
            content,
            "- new",
            "  ## Log  ",
            InsertPosition::Bottom,
            false,
        );
        assert!(result.contains("- entry"));
        assert!(result.contains("- new"));
    }
}
