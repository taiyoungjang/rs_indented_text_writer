use std::iter;
use std::io::Write;
use std::string::FromUtf8Error;

const DEFAULT_CAPACITY: usize = 1024;
const MAX_UNICODE_WIDTH: usize = 4;

#[cfg(windows)]
const LINE_ENDING: &'static [u8] = b"\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static [u8] = b"\n";

/// This is a growable string builder.
#[derive(Debug)]
pub struct IndentedTextWriter {
    inner: Vec<u8>,
    tab_string: Vec<u8>,
    indent_level: i32,
    tabs_pending: bool,
}

impl Default for IndentedTextWriter {
    fn default() -> IndentedTextWriter {
        let inner = Vec::with_capacity(DEFAULT_CAPACITY);
        let tab_string = " ".to_bytes();
        let indent_level = 0;
        let tabs_pending = false;
        IndentedTextWriter {
            inner,
            tab_string,
            indent_level,
            tabs_pending,
        }
    }
}

impl IndentedTextWriter {
    /// Return a new `Builder` with an initial capacity.
    pub fn new(size: usize, tab_string: &String) -> IndentedTextWriter {
        let inner = Vec::with_capacity(size);
        IndentedTextWriter {
            inner,
            tab_string: tab_string.to_bytes(),
            indent_level: 0,
            tabs_pending: false,
        }
    }
    fn output_tabs(&mut self) {
        if self.tabs_pending {
            for _ in 1..=self.indent_level {
                self.inner.write_all(&self.tab_string).unwrap()
            }
            self.tabs_pending = false;
        }
    }
    /// indent
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    /// indent
    pub fn indents(&mut self, len: i32) {
        self.indent_level += len;
    }
    /// unindent
    pub fn unindent(&mut self) {
        self.indent_level -= 1;
    }

    /// unindent
    pub fn unindents(&mut self, size: i32) {
        self.indent_level -= size;
    }

    /// Add a type that can be viewed as a slice of bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indented_text_writer::IndentedTextWriter;
    ///
    /// let mut writer = IndentedTextWriter::default();
    /// writer.write("some string");
    /// ```
    pub fn write<T: ToBytes>(&mut self, buf: T) {
        self.output_tabs();
        self.inner.write_all(&buf.to_bytes()).unwrap()
    }

    /// Add a type that can be viewed as a slice of bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indented_text_writer::IndentedTextWriter;
    ///
    /// let mut writer = IndentedTextWriter::default();
    /// writer.write_no_tabs("some string");
    /// ```
    pub fn write_no_tabs<T: ToBytes>(&mut self, buf: T) {
        self.inner.write_all(&buf.to_bytes()).unwrap()
    }

    /// Add a type that can be viewed as a slice of bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indented_text_writer::IndentedTextWriter;
    ///
    /// let mut writer = IndentedTextWriter::default();
    /// writer.write_line("some string");
    /// ```
    pub fn write_line<T: ToBytes>(&mut self, buf: T) {
        self.output_tabs();
        self.inner.write_all(&buf.to_bytes()).unwrap();
        self.inner.write_all(LINE_ENDING).unwrap();
        self.tabs_pending = true;
    }

    /// Add a type that can be viewed as a slice of bytes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indented_text_writer::IndentedTextWriter;
    ///
    /// let mut writer = IndentedTextWriter::new();
    /// writer.write_line_no_tabs("some string");
    /// ```
    pub fn write_line_no_tabs<T: ToBytes>(&mut self, buf: T) {
        self.inner.write_all(&buf.to_bytes()).unwrap();
        self.inner.write_all(LINE_ENDING).unwrap()
    }

    /// Return the current length in bytes of the underlying buffer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indented_text_writer::IndentedTextWriter;
    ///
    /// let mut writer = IndentedTextWriter::default();
    /// writer.write("four");
    /// assert_eq!(writer.len(), 4);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Return a `String` of our buffer once we are done appending to it. This method consumes
    /// the underlying buffer.
    ///
    /// # Example
    ///
    /// ```rust
    /// use indented_text_writer::IndentedTextWriter;
    ///
    /// let mut writer = IndentedTextWriter::default();
    /// writer.write("i am building");
    /// writer.write(' ');
    /// writer.write("a string");
    /// assert_eq!(writer.string().unwrap(), "i am building a string");
    /// ```
    pub fn string(self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.inner)
    }
}

/// A trait to convert a value into a byte slice that can be appended to a `Builder`.
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

// Generate a buffer with the same length as the given argument in order to use `copy_from_slice`.
fn make_copyable_buf(len: usize) -> Vec<u8> {
    iter::repeat(0).take(len).collect::<Vec<u8>>()
}

// Copy the slice into a `Vec` with the same capacity.
fn slice_to_vec(s: &[u8]) -> Vec<u8> {
    let mut res = make_copyable_buf(s.len());
    res.copy_from_slice(s);
    res
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        slice_to_vec(self.as_bytes())
    }
}

impl<'a> ToBytes for &'a str {
    fn to_bytes(&self) -> Vec<u8> {
        slice_to_vec(self.as_bytes())
    }
}

impl ToBytes for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl ToBytes for char {
    fn to_bytes(&self) -> Vec<u8> {
        // The maximum length of a unicode character is 4 bytes.
        let mut buf = [0; MAX_UNICODE_WIDTH];
        slice_to_vec(self.encode_utf8(&mut buf).as_bytes())
    }
}

impl<'a> ToBytes for &'a [u8] {
    fn to_bytes(&self) -> Vec<u8> {
        slice_to_vec(self)
    }
}

#[cfg(test)]
mod tests {
    use super::IndentedTextWriter;

    #[test]
    fn test_all_writes() {
        let mut b = IndentedTextWriter::default();
        b.write(String::from("hello"));
        b.write_line(',');
        assert_eq!(b.string().unwrap(), "hello, world it works");
    }
}
