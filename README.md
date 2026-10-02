# plank-tv

The [Turbo Vision](https://github.com/aovestdipaperino/turbo-vision-4-rust) <->
[plank](https://github.com/aovestdipaperino/plank) glue, plus the csvedit
plugin built on it: a full-screen CSV table editor that runs inside plank as
a WASM `frame` component (`dev.plank.csvedit`).

The editor itself — the document model, the dialogs, the commands, the
session that drives them — is generic and lives in the published
[`tv-extensions`](https://github.com/aovestdipaperino/tv-extensions) crate's
`csv` feature (`tv_extensions::csv`). What stays in this crate is plank's own
glue:

- `src/frame.rs`: the Extism exports (`frame_open`, `frame_key`,
  `frame_step`, `frame_close`, the `edit_csv` tool and the `/csvedit` slash
  command), driving one `tv_extensions::csv::Session`.
- `src/paint.rs`: a session's cell buffer, and its text cursor, as plank's
  `CellGlyph`s. plank's frame protocol has no cursor field, so the cursor
  cell is drawn with its foreground and background swapped instead.
- `src/keys.rs`: `payload_text`, reading plank's key-payload JSON
  (`tv_extensions::keys::translate` does the rest: turning a key code into a
  Turbo Vision event).
- `src/disk.rs`: `PlankDisk`, a `tv_extensions::csv::Disk` over plank's RAM
  disk (the `fs` capability's host functions).
- `src/summary.rs`: the row-count summary the `edit_csv` tool reports when
  it closes.

It works on a private scratch disk the host lends it, and plank opens it
three ways: as a slash command (`/csvedit:new`), as the `edit_csv` tool the
model can call on a file, and as the grid behind a profile's MCP tables (the
ChatBGT profile routes its transactions, categories, rules and budgets
through it) — which is why `frame_open` always uses
`Session::open_bridged`: when the loaded document's first header cell is
`#`, that keeps the grid's shape and name host-owned. The host side of those
paths is described in plank's `docs/WASM-PLUGINS.md`.

## Building

plank-tv targets `wasm32-wasip1` rather than `wasm32-unknown-unknown`,
because Turbo Vision reads the clock and that needs WASI.

```sh
rustup target add wasm32-wasip1
sh package.sh
```

That writes `dist/csvedit/`, an installable plugin directory, plus
`dist/plank-csvedit.tar.gz` and `dist/SHA256SUMS`. Install the directory or
the tarball with `/plugins install`, then approve the module with
`/plugins trust dev.plank.csvedit`. The approval is recorded against the
module's SHA-256, so it is asked again only when the bytes change.

`cargo test --lib` runs the pure modules natively; the Extism exports in
`src/frame.rs` compile only for wasm, and only with the default `csvedit`
feature (`cargo build --target wasm32-wasip1 --no-default-features` builds
the glue as a plain library, with no exports, for a second plugin to depend
on). plank's own `tests/wasm_csvedit.rs` and `tests/grid_bridge.rs` drive the
built module through the real host, loading it from
`target/wasm32-wasip1/release/plank_tv.wasm`.

## Dependencies

- [turbo-vision](https://crates.io/crates/turbo-vision), without its native
  terminal backend
- [tv-extensions](https://crates.io/crates/tv-extensions) (feature `csv`),
  for the CSV editor itself, the host-driven backend, key translation and
  the event pump
- [plank-guest-support](https://github.com/aovestdipaperino/plank-guest-support),
  for glyph packing and the `OpenParams` readers (pinned by git rev)

## License

MIT
