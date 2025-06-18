/*
fn main() {
    let reference = dangle(); // ERROR
}

fn dangle() -> &String {
    let s = String::from("Halo");
    &s // ERROR: s akan dihapus saat fungsi selesai, referensi tidak valid
}
*/