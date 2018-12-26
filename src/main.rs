extern crate flate2;

use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

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

    println!("Playing with a HashMap");
    println!("----------------------");
    let mut hash: HashMap<String, u32> = HashMap::new();
    hash.insert("René".to_string(), 10);
    hash.insert("Bla".into(), 12);
    hash.entry("René".to_owned()).or_insert(25); // Already exist
    hash.entry("TOTO".to_string()).or_insert(25);

    for (key, value) in hash.iter() {
        println!("Key: {}={}", key, value);
    }

    if hash.contains_key("Bla") {
        println!("Key: Bla is in the HashMap");
        println!(
            "Value of key: Bla {}",
            hash.get("Bla").expect("Key not found !")
        );
    }
}
