use rust_dos::*;

#[allow(dead_code)]
pub(crate) fn file_read_test() {
    let test_file = dos::file::File::open("C:\\AUTOEXEC.BAT");
    let test_file = test_file.unwrap_or(dos::file::File::open("README.md").unwrap());
    let mut buffer = [0; 100];
    let bytes_read = test_file.read(&mut buffer).unwrap();
    println!("{} bytes read", bytes_read);
    println!("{}", core::str::from_utf8(&buffer).unwrap());
    match test_file.close() {
        Ok(_) => println!("File closed"),
        Err(_) => println!("Error closing file")
    }
}

#[allow(dead_code)]
pub(crate) fn file_attribute_test() {
    let attributes = dos::file::File::attributes("C:\\AUTOEXEC.BAT");
    let attributes = attributes.unwrap_or(dos::file::File::attributes("README.md").unwrap());

    // Attributes aren't supported in DOSBox so expect this to have no info
    println!("Attributes: {:?}", attributes);

    println!("Long filename {:?}", dos::file::File::attributes("Really long name or something"));
}

#[allow(dead_code)]
pub(crate) fn directory_test() {
    let path = "C:\\1A2B3C4D";

    print!("Creating folder {path}... ");
    dos::file::Directory::make(path).unwrap();
    println!("Done");

    print!("Deleting folder {path}... ");
    dos::file::Directory::remove(path).unwrap();
    println!("Done");

}