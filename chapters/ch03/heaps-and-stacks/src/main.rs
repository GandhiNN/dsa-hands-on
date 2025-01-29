use std::mem;

struct MyStruct {
    a: u8, // 1 byte
    b: u8, // 1 byte
    c: u8, // 1 byte
}

fn main() {
    assert_eq!(mem::size_of::<MyStruct>(), 3 * mem::size_of::<u8>());
    assert_eq!(mem::size_of::<[MyStruct; 2]>(), 3 * mem::size_of::<u8>() * 2);

    println!("Size of single MyStruct: {}", mem::size_of::<MyStruct>());
    println!("Size of array of 2 MyStructs: {}", mem::size_of::<[MyStruct; 2]>());
}
