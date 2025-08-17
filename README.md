# fslint

`fslint` is a file linter (alpha stage — work in progress) that scans the current directory (or `--path`) using `fslint.json`, checking only files matching the `include` patterns.

### rules

See full list of supported [linter rules](./linter-rules.md).

### installation

```zsh
$ cargo install fslint
```

### usage

```zsh
$ fslint --help
Linter for filesystem consistency: names, metadata, permissions and sizes.

Usage: fslint [OPTIONS]

Options:
  -p, --path <PATH>  [default: .]
  -v, --verbose...
  -s, --schema
  -h, --help         Print help
  -V, --version      Print version
```

### config schema

Validate your config against [schema.json](https://raw.githubusercontent.com/grubyak/fslint/main/schema.json) or generate a local one with:

```zsh
$ fslint -s > schema.json
```

<!-- [schemastore](https://www.schemastore.org/fslint.json) -->

To enable schema validation in your editor, add the following to your `fslint.json`:

```json
"$schema": "https://raw.githubusercontent.com/grubyak/fslint/main/schema.json"
```

### sample config

```json
{
  "$schema": "https://raw.githubusercontent.com/grubyak/fslint/main/schema.json",
  "entries": [
    {
      "include": ["**/*.cr3"],
      "rules": {
        "exif-has-coords": {
          "level": "warn",
          "latitude": true,
          "longitude": true
        },
        "exif-has-capture-datetime": {
          "level": "error",
          "date": true
        }
      }
    },
    {
      // ...
    }
  ]
}
```

### output

```
$ fslint
/Users/foo/Desktop/not-an-image.cr3
    error exif-has-capture-datetime: exif metadata is missing
    error exif-has-coords: exif metadata is missing

/Users/foo/Desktop/test.cr3
    warn  exif-has-coords: missing latitude

✖ 3 problems (2 errors, 1 warning) -- inspected 2 files, skipped 18031 [130.25ms]
```

### debugging (verbosity)

```zsh
$ fslint -v    # logs
$ fslint -vv   # more logs
$ fslint -vvv  # max
```

### coffee

If you find this useful, consider [buying me a coffee](https://coff.ee/grubyak).

<a href="https://coff.ee/grubyak">
  <img src="bmc.png" alt="buy me a coffee" width="200"/>
</a>
