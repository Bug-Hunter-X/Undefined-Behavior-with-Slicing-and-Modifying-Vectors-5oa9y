fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let slice = &vec[1..];

    // This is UB if vec is modified after creating the slice
    for i in slice {
        println!("{}", i);
    }
}