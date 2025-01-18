fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Create a copy to avoid UB when modifying the original vector
    let slice = vec[1..].to_vec();

    // Modify the original vector (This is safe now)
    vec.push(4);

    for i in slice {
        println!("{}", i);
    }
} 