# uhttp\_chunked\_write

[Documentation](https://docs.rs/uhttp_chunked_write)

This crate implements a zero-copy, zero-allocation writer for HTTP [chunked response
bodies](https://tools.ietf.org/html/rfc7230#section-4.1). The result can be written
directly into a
[`TcpStream`](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html) or any
other object that implements
[`Write`](https://doc.rust-lang.org/stable/std/io/trait.Write.html).

## Example

```rust
use uhttp_chunked_write::ChunkedWrite;
use std::io::Write;

let mut buf = [0; 25];

{
    let mut body = ChunkedWrite::new(&mut buf[..]);
    write!(&mut body, "hello {}", 1337).unwrap();
}

assert_eq!(&buf[..], &b"6\r\nhello \r\n4\r\n1337\r\n0\r\n\r\n"[..]);
```

## Usage

This [crate](https://crates.io/crates/uhttp_chunked_write) can be used through cargo by
adding it as a dependency in `Cargo.toml`:

```toml
[dependencies]
uhttp_chunked_write = "0.5.0"
```
and importing it in the crate root:

```rust
extern crate uhttp_chunked_write;
```
