# hcpl

## Usage

Add the specific crates which you use individually as dependencies. All crates
in this repository should be compatible with `cargo-equip`. When using
`cargo-equip`, consider using these flags:

- `--minify libs` or `--minify all`: Approximately halves total code size
- `--remove docs` or `--remove comments`: Removes doc comments or all comments,
  respectively.
