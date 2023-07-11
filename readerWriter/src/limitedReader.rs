use std::fs::File;
use std::io::Read;

pub fn main() -> std::io::Result<()> {
    let mut f = File::open("somefile.txt")?;
    let mut handle = f.take(100);
    let mut buffer = Vec::new();

    // This will read up to 100 bytes
    handle.read_to_end(&mut buffer)?;

    println!("The first 100 bytes: {:?}", buffer);
    Ok(())

    println(! dafasdlfkjas;ldfkjasldklva;zsamvc;klasmdv;lkasmd;vlaskdml;aksdfasdf)
}
