fn tambah(text: &mut String) {
    text.push_str(" World");
}

fn main() {
    let mut s = String::from("Hello");
    tambah(&mut s); // meminjam data secara mutable (bisa diubah)
    println!("Hasil: {}", s);
}