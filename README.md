# string-builder

This crate is a simple string builder type allowing you to append anything that satisfies the
`ToBytes` trait to it. This includes things such as string slices, owned strings, byte slices,
and characters for example.

like : [C#](https://learn.microsoft.com/en-us/dotnet/api/system.codedom.compiler.indentedtextwriter) 
## Example
```rust
extern crate indented_text_writer;

use indented_text_writer::IndentedTextWriter;

fn main() {
    let mut b = IndentedTextWriter::default();
    b.write_line("hello world");
    b.indents(2);
    b.write_line("hello ");
    b.unindents(2);
}
```

forked by https://github.com/gsquire/string-builder
## License
MIT