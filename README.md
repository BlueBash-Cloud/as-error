# README

With the `file_error` feature (included by default), standard I/O errors convert
to `AsError::IoError` using `?` or `.into()`:

```rust
use as_error::ResultExt;

fn read_config(path: &str) -> ResultExt<String> {
    Ok(std::fs::read_to_string(path)?)
}
```

The original `std::io::Error` is preserved as the error source. Actix responses
use HTTP 500 and the generic `FILE_ERROR` code without exposing I/O details.
Use the existing `FileError` variants for specific file operation error responses.

## TODO

- [ ] Implement error log method
