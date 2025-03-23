
mod array;
fn main() {
    let arr = [1, 3, 4, 6, 8];
    match array::binary_search(&arr, 6) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }

    match array::binary_search(&arr, 5) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
