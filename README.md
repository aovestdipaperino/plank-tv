# plank-csvedit

A full-screen [Turbo Vision](https://github.com/aovestdipaperino/turbo-vision-4-rust)
CSV table editor that runs inside [plank](https://github.com/aovestdipaperino/plank)
as a WASM `frame` component (`dev.plank.csvedit`).

It works on a private scratch disk the host lends it, and plank opens it three
ways: as a slash command (`/csvedit:new`), as the `edit_csv` tool the model can
call on a file, and as the grid behind a profile's MCP tables (the ChatBGT
profile routes its transactions, categories, rules and budgets through it).
The host side of those paths is described in plank's `docs/WASM-PLUGINS.md`.

## Building

csvedit targets `wasm32-wasip1` rather than `wasm32-unknown-unknown`, because
Turbo Vision reads the clock and that needs WASI.

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
`src/frame.rs` compile only for wasm. plank's own `tests/wasm_csvedit.rs` and
`tests/grid_bridge.rs` drive the built module through the real host.

## Dependencies

All pinned by git rev in `Cargo.toml`:

- [turbo-vision-4-rust](https://github.com/aovestdipaperino/turbo-vision-4-rust),
  without its native terminal backend
- [tv-extensions](https://github.com/aovestdipaperino/tv-extensions), for the
  host-driven backend, the event pump and the separator grid
- [plank-guest-support](https://github.com/aovestdipaperino/plank-guest-support),
  for glyph packing and the `OpenParams` readers

## License

MIT
