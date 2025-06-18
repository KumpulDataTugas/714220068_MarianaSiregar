fn main() {
    let s1 = String::from("Mariana");
    let s2 = s1; // s1 pindah ke s2
    // println!("{}", s1); // ERROR: s1 tidak bisa dipakai lagi
    println!("{}", s2);
}