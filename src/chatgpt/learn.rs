// import
use std::io;
use std::io::Write;

pub fn run() {
    println!("Halo, selamat belajar Rust!");
    println!("Mari kita buat sesuatu yang menyenangkan bersama!");

    // 1. Tipe Data: Signed dan Unsigned Integer
    let unsigned: u8 = 255; // Nilai maksimum untuk u8
    let signed: i8 = -128; // Nilai minimum untuk i8
    println!("\nTipe data integer:");
    println!("Unsigned integer (u8): {}", unsigned);
    println!("Signed integer (i8): {}", signed);

    // 2. Floating-Point Numbers
    let pi: f32 = 3.14; // f32
    let e: f64 = 2.718281828459045; // f64 (default)
    println!("\nTipe data floating-point:");
    println!("Pi (f32): {}", pi);
    println!("Euler's number (f64): {}", e);

    // 3. Operasi Matematika
    let a = 10;
    let b = 3;
    println!("\nOperasi matematika:");
    println!("Penjumlahan ({} + {}): {}", a, b, a + b);
    println!("Pengurangan ({} - {}): {}", a, b, a - b);
    println!("Perkalian ({} * {}): {}", a, b, a * b);
    println!("Pembagian ({} / {}): {}", a, b, a / b);
    println!("Modulus ({} % {}): {}", a, b, a % b);

    // 4. Random Number (Angka Acak)
    println!("\nAngka acak:");
    use rand::Rng; // Pastikan kamu sudah menambahkan crate `rand` di `Cargo.toml`
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(1..=100);
    println!("Angka acak (1-100): {}", random_number);

    // 5. Tantangan: Tebak Angka
    println!("\nTebak angka! (Fitur ini akan kita buat di latihan berikutnya)");
}

// test input for basic 5.tantangan tebak angka
#[test]
fn input_test() {
    let mut input = String::new();
    print!("Masukkan sesuatu: "); // Tidak menambahkan newline
    io::stdout().flush().unwrap(); // Pastikan output langsung muncul di terminal

    io::stdin()
        .read_line(&mut input)
        .expect("Gagal membaca input");

    println!("Kamu memasukkan: {}", input.trim());
}
