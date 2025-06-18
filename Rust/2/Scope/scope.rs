fn main() {
    {
        let nama = "Rustacean";
        println!("Di dalam scope: {}", nama);
    }
    // println!("{}", nama); // ERROR: nama sudah di-drop
}