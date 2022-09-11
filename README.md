# shasher
The `shasher` library is used to compute the SHA-256 hash for a givien string or a file specifiled by a given file path.

## Usage
Add this to your `Cargo.toml`
```
[dependencies]
shasher = { git = "https://github.com/Quin-Darcy/shasher", branch = "master" }
```
There one method in this library: 
* `get_hash(flag: u8, data: &str) -> String`
The flag is either 102 (ASCII 'f') or 115 (ASCII 's'). 

## Examples
### Code #1:
```
use shasher;

fn main() {
  let string_flag: u8 = 115;
  let file_flag: u8 = 102;
  let string_data: &str = "this is a test string";
  let file_data: &str = "src/main.rs"
  
  println!("{}", shasher::get_hash(string_flag, string_data));
  println!("{}", shasher::get_hash(file_flag, file_data));
}
```
### Output
```
f6774519d1c7a3389ef327e9c04766b999db8cdfb85d1346c471ee86d65885bc
2762b2052be0cc5898769a8ce8435ffa7e576ddc0ed57572d8663ed5e5ae59e   
```

