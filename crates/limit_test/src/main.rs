use std::fs::File;
use std::io::Write;

fn main() {
    let mut file =
        File::create("target/testpack/data/testpack/function/limit_test.mcfunction").unwrap();
    for i in 0..100000 {
        writeln!(file, "say {}", i).unwrap();
    }
}
