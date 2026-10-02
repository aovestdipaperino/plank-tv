//! The turbo-vision <-> plank glue, plus the csvedit plugin it carries.
//!
//! The generic CSV editor itself (`Session`, `CsvDoc`, `Disk`, the dialogs
//! and commands) lives in [`tv_extensions::csv`], published separately. What
//! stays here is plank-specific: `keys::payload_text` reads plank's
//! key-payload JSON, `paint` turns a session's cell buffer (and its cursor)
//! into plank's `CellGlyph`s, `disk::PlankDisk` is a [`tv_extensions::csv::Disk`]
//! over plank's RAM disk, and `frame` holds the Extism exports that make
//! this the `dev.plank.csvedit` plugin. `frame` is compiled only for wasm,
//! and only with the `csvedit` feature, so native `cargo test` covers
//! everything but the ABI glue, and a second plugin can depend on this
//! crate as a library (`default-features = false`) without its exports.

pub mod disk;
#[cfg(all(target_arch = "wasm32", feature = "csvedit"))]
pub mod frame;
pub mod keys;
pub mod paint;
pub mod summary;

#[cfg(test)]
mod tests {
    //! Native coverage for the one plank-specific choice `frame.rs` makes
    //! that isn't itself testable natively (it's wasm- and feature-gated):
    //! opening every session with [`tv_extensions::csv::Session::open_bridged`],
    //! never `open`. That one call is what makes plank's grid-bridge mode
    //! (the MCP tables' `#`-headed CSVs) refuse reshape and switch-document
    //! commands; `open` never bridges. These drive the same `Session` API
    //! `frame_open`/`frame_key` do, standing in for the plugin_fn glue.
    use turbo_vision::core::command::CommandId;
    use turbo_vision::core::event::Event;
    use tv_extensions::csv::{Disk, MemDisk, Session};

    fn disk_with(files: &[(&str, &str)]) -> Box<dyn Disk> {
        let mut disk = MemDisk::default();
        for (name, text) in files {
            Disk::write(&mut disk, name, text).unwrap();
        }
        Box::new(disk)
    }

    fn command(s: &mut Session, c: CommandId) -> Option<String> {
        s.key(Event::command(c))
    }

    #[test]
    fn open_bridged_refuses_reshape_on_a_hash_headed_grid() {
        use tv_extensions::csv::commands::CMD_SAVE_AS;
        let disk = disk_with(&[("grid.csv", "#,name\n1,alice\n2,bob\n")]);
        let mut s = Session::open_bridged(80, 24, "grid.csv", disk);
        s.step(80, 24);
        assert!(command(&mut s, CMD_SAVE_AS).is_none());
        s.step(80, 24);
        let text: String = s.buffer().iter().flatten().map(|c| c.ch).collect();
        assert!(text.contains("this grid stays on grid.csv"), "{text}");
    }

    #[test]
    fn open_bridged_leaves_an_ordinary_grid_unaffected() {
        use tv_extensions::csv::commands::CMD_SAVE_AS;
        let disk = disk_with(&[("plain.csv", "name\nalice\nbob\n")]);
        let mut s = Session::open_bridged(80, 24, "plain.csv", disk);
        s.step(80, 24);
        assert!(command(&mut s, CMD_SAVE_AS).is_none());
        s.step(80, 24);
        let text: String = s.buffer().iter().flatten().map(|c| c.ch).collect();
        assert!(!text.contains("this grid stays on"), "{text}");
        assert!(text.contains("Save as"), "{text}");
    }

    /// The scrollback line `frame_close` reports for the `edit_csv` tool
    /// path (via `tool_resume`/`summary::resume_line`) names the file it
    /// saved to and its final shape; this is the line a bridged grid-bridge
    /// close produces, which plank's host matches against to know the save
    /// went through.
    #[test]
    fn a_bridged_save_reports_the_plank_close_line() {
        let disk = disk_with(&[("grid.csv", "#,name\n1,alice\n2,bob\n")]);
        let mut s = Session::open_bridged(80, 24, "grid.csv", disk);
        s.step(80, 24);
        use tv_extensions::csv::commands::CMD_EXIT;
        let line = command(&mut s, CMD_EXIT).expect("clean exit closes at once");
        assert_eq!(line, "csvedit: closed without saving");
    }
}
