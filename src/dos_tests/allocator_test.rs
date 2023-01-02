use rust_dos::*;
use dos::*;

#[allow(dead_code)]
pub(crate) fn allocator_test() {
    let mut box1 = Box::new(5);
    assert_eq!(*box1, 5);
    *box1 = 10;
    assert_eq!(*box1, 10);

    let mut string1 = String::from("konnichiwa");
    assert_eq!(string1, "konnichiwa");
    string1 += " sekai";
    assert_eq!(string1, "konnichiwa sekai");
    let string2 = String::from("こんにちわ 世界!");
    string1 += "! ";
    string1 += &*string2;
    assert_eq!(string1, "konnichiwa sekai! こんにちわ 世界!");  // Don't try to print this in the console, the result is weird.

    let mut vec1 = vec![12; 200];
    assert_eq!(vec1.len(), 200);
    for i in 0..200 {
        assert_eq!(vec1[i], 12);
    }
    vec1.push(13);
    assert_eq!(vec1.len(), 201);
    assert_eq!(vec1[200], 13);
    vec1.resize(300, 14);
    assert_eq!(vec1.len(), 300);
    for i in 0..200 {
        assert_eq!(vec1[i], 12);
    }
    assert_eq!(vec1[200], 13);
    for i in 201..300 {
        assert_eq!(vec1[i], 14);
    }
    vec1.resize(10, 15);
    assert_eq!(vec1.len(), 10);
    for i in 0..10 {
        assert_eq!(vec1[i], 12);
    }
}