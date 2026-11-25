use std::io::{self, Write};

fn convert(input: &str, from: u32, to: u32) -> Result<String, String> {
    let cleaned = input.trim().replace('_', "");
    if cleaned.is_empty() {
        return Err("Input tidak boleh kosong.".into());
    }
    let value = i128::from_str_radix(&cleaned, from)
        .map_err(|_| format!("Input '{}' bukan angka valid pada basis {}.", input.trim(), from))?;
    Ok(format_radix(value, to))
}

fn format_radix(mut n: i128, radix: u32) -> String {
    if n == 0 { return "0".to_string(); }
    let negative = n < 0;
    if negative { n = -n; }
    let digits = b"0123456789abcdef";
    let mut out = Vec::new();
    while n > 0 {
        out.push(digits[(n % radix as i128) as usize] as char);
        n /= radix as i128;
    }
    out.reverse();
    let result: String = out.into_iter().collect();
    if negative { format!("-{}", result) } else { result }
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn main() {
    println!("=== RADIX CONVERTER ===");
    println!("Konversi angka: desimal, hex, oktal, dan biner\n");

    loop {
        println!("Pilih sumber angka:");
        println!("1. Desimal");
        println!("2. Hexadecimal");
        println!("3. Oktal");
        println!("4. Biner");
        println!("0. Keluar");

        let choice = match read_line("Pilihan: ") {
            Ok(s) => s,
            Err(e) => { eprintln!("Gagal membaca input: {e}"); break; }
        };
        let from = match choice.as_str() {
            "1" => 10, "2" => 16, "3" => 8, "4" => 2, "0" => break,
            _ => { println!("Pilihan tidak valid.\n"); continue; }
        };

        println!("Pilih basis tujuan:");
        println!("1. Desimal");
        println!("2. Hexadecimal");
        println!("3. Oktal");
        println!("4. Biner");
        let dest = match read_line("Pilihan: ") {
            Ok(s) => s,
            Err(e) => { eprintln!("Gagal membaca input: {e}"); break; }
        };
        let to = match dest.as_str() {
            "1" => 10, "2" => 16, "3" => 8, "4" => 2,
            _ => { println!("Pilihan tujuan tidak valid.\n"); continue; }
        };

        let number = match read_line("Masukkan angka: ") {
            Ok(s) => s,
            Err(e) => { eprintln!("Gagal membaca input: {e}"); break; }
        };
        match convert(&number, from, to) {
            Ok(result) => println!("Hasil: {}\n", result),
            Err(err) => println!("Error: {}\n", err),
        }
    }
    println!("Terima kasih!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_to_other_bases() {
        assert_eq!(convert("255", 10, 16).unwrap(), "ff");
        assert_eq!(convert("10", 10, 2).unwrap(), "1010");
        assert_eq!(convert("64", 10, 8).unwrap(), "100");
    }

    #[test]
    fn other_bases_to_decimal() {
        assert_eq!(convert("ff", 16, 10).unwrap(), "255");
        assert_eq!(convert("1010", 2, 10).unwrap(), "10");
        assert_eq!(convert("100", 8, 10).unwrap(), "64");
    }

    #[test]
    fn negative_values() {
        assert_eq!(convert("-10", 10, 2).unwrap(), "-1010");
    }
}