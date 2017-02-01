//! This crate implements a zero-copy, zero-allocation writer for HTTP [chunked response
//! bodies](https://tools.ietf.org/html/rfc7230#section-4.1). The result can be written
//! directly into a
//! [`TcpStream`](https://doc.rust-lang.org/stable/std/net/struct.TcpStream.html) or any
//! other object that implements
//! [`Write`](https://doc.rust-lang.org/stable/std/io/trait.Write.html).
//!
//! ## Example
//!
//! ```rust
//! use uhttp_chunked_write::ChunkedWrite;
//! use std::io::Write;
//!
//! let mut buf = [0; 25];
//!
//! {
//!     let mut body = ChunkedWrite::new(&mut buf[..]);
//!     write!(&mut body, "hello {}", 1337).unwrap();
//! }
//!
//! assert_eq!(&buf[..], &b"6\r\nhello \r\n4\r\n1337\r\n0\r\n\r\n"[..]);
//! ```

use std::io::Write;

/// Writes bytes in the HTTP chunked encoding protocol.
///
/// When the object goes out of scope the chunked message is terminated and the stream is
/// flushed.
///
/// To reduce the number of write syscalls to the underlying stream when using `write!` or
/// byte-based serialization, wrap the object in a
/// [`BufWriter`](https://doc.rust-lang.org/stable/std/io/struct.BufWriter.html), for
/// example `BufWriter::new(ChunkedWrite::new(stream))`.
pub struct ChunkedWrite<W: Write>(W);

impl<W: Write> ChunkedWrite<W> {
    /// Create a new `ChunkedWrite` to write into the given stream.
    pub fn new(sink: W) -> Self {
        ChunkedWrite(sink)
    }

    /// Send the given data in chunked encoding.
    fn send(&mut self, data: &[u8]) -> std::io::Result<()> {
        try!(write!(self.0, "{:x}\r\n", data.len()));
        try!(self.0.write_all(data));
        try!(write!(self.0, "\r\n"));

        Ok(())
    }
}

impl<W: Write> Write for ChunkedWrite<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        try!(self.send(buf));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> { self.0.flush() }
}

impl<W: Write> Drop for ChunkedWrite<W> {
    fn drop(&mut self) {
        // Send terminating empty chunk and flush the stream.
        self.send(&[]).is_ok();
        self.flush().is_ok();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_chunked_write() {
        let mut buf = [0; 32];

        {
            let mut w = ChunkedWrite::new(&mut buf[..]);
            w.write_all(b"abc def").unwrap();
            w.write_all(b"gh\nijklmno").unwrap();
        }

        assert_eq!(&buf[..], b"7\r\nabc def\r\na\r\ngh\nijklmno\r\n0\r\n\r\n");
    }
}
