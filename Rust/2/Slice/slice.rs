fn main() {
    let kalimat = String::from("Hello Rustacean!");
    let hello = &kalimat[0..5]; // ambil bagian awal
    println!("Slice: {}", hello); // Output: Hello
}