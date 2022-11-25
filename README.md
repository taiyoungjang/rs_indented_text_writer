# string-builder

This crate is a simple string builder type allowing you to append anything that satisfies the
`ToBytes` trait to it. This includes things such as string slices, owned strings, byte slices,
and characters for example.

like : [C#](https://learn.microsoft.com/en-us/dotnet/api/system.codedom.compiler.indentedtextwriter) 
## Example
write code:
```rust
#[cfg(test)]
use super::IndentedTextWriter;

fn main() {
    let mut writer = IndentedTextWriter::new("\t",1024);
    writer.write_line("struct Data {");
    writer.indents(1);
    writer.write_line("name: String,");
    writer.write_line("value: i32");
    writer.unindents(1);
    writer.write_line("}");
    println!("{}",writer.string().unwrap());
}
```
Result:
```rust
struct Data {
	name: String,
	value: i32
}
```

forked by https://github.com/gsquire/string-builder
## License
MIT