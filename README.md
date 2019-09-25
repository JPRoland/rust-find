# Rust Find

A simple utility similar to the Unix `find` command built with Rust. `rust_find` recursively searches the provided directory for any files that match the supplied regex pattern.

## Usage

```bash
./rust_find <pattern> <directory>
```

Example:

```bash
# Finds all files ending in `.rs` in the `/src` folder and all sub folders.
./rust_find ".*\.rs" ./src
```
