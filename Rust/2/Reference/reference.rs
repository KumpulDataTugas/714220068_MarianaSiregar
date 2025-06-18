fn print_ref(text: &String) {
    println!("Isi: {}", text);
}

fn main() {
    let s = String::from("Hello");
    print_ref(&s); // meminjam s secara aman (tanpa mengambil kepemilikan)
    println!("Tetap bisa dipakai: {}", s);
}