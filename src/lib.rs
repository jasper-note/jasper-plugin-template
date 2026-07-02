//! Template plugin: trims trailing whitespace from every line before a note is saved.
//!
//! Replace this with your logic. `register!` accepts any combination of the three
//! slots — `before_save`, `storage` (a type implementing `sdk::storage::Storage`)
//! and `command` — see the README and the reference plugins in the main repo.

use jasper_plugin_sdk as sdk;
use sdk::core::model::Note;

fn trim_trailing(body: &str) -> String {
	body.lines().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n")
}

fn before_save(mut note: Note) -> Result<Note, String> {
	note.body = trim_trailing(&note.body);
	Ok(note)
}

sdk::register! { before_save: before_save }

// Plain unit tests compile and run natively (`cargo test`) — the wasm ABI exports
// are only emitted for the wasm32 target.
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn trims_trailing_whitespace() {
		assert_eq!(trim_trailing("a  \nb\t\nc"), "a\nb\nc");
	}

	#[test]
	fn keeps_leading_whitespace() {
		assert_eq!(trim_trailing("  indented  "), "  indented");
	}
}
