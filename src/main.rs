extern crate flate2;

use std::io;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use flate2::read::GzDecoder;

fn run() -> Result<String, io::Error> {
    let path = "ascii.txt.gz";

    // Open a compressed tarball
    let gz = File::open(path)?;
    // Decompress it
    let mut uncompress = GzDecoder::new(gz);
    let mut contents = String::new();
    uncompress.read_to_string(&mut contents)?;
    //let mut archive = Archive::new(tar);
    // Unpack the archive inside curent working directory
    //archive.unpack(".")?;

    Ok(contents)
}

fn foo(s: String) -> Result<String, io::Error> {
    let mut file = File::create("foo.txt")?;
    file.write_all(&s.into_bytes())?;
    Ok("ok".to_string())
}

fn foo2() -> Result<String, io::Error> {
    let mut file = File::open("foo.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    println!("Hello, world!");
    let _wrt = foo("Coucou".to_string());
    let rd = foo2();
    match rd {
        Err(e) => panic!("Issue {:?}", e),
        Ok(s) => println!("{}", s),
    };
    let result = run().expect("Failed");
    println!("{}", result);
}
