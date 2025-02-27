use rand::{rng, Rng};
use std::io::{self, Write};

// Cтруктура для хранения цветов
struct ColorRgb(u32, u32, u32);

fn main() {
    println!("Привет!");
    println!("Это генератор цветов.");
    loop {
        println!("Введите 1 если в RGB.");
        println!("Введите 2 если в HEX.\n");
        println!("Введите 0 если хотите выйти.\n");
        print!("Выберете формат: ");

        // Сбрасываю буффер
        io::stdout().flush().unwrap();

        // Читаю ввод
        let mut format = String::new();
        io::stdin().read_line(&mut format).expect("Failed input.");

        // Парсинг ввода
        if let Ok(format) = format.trim().parse::<i32>() {
            match format {
                // Выходим из цикла
                0 => break,
                // Печатаю цвет в RGB формате
                1 => {
                    let rgb = rand_rgb();
                    println!(
                        "Color in rgb: \x1b[31m{}\x1b[0m \x1b[32m{}\x1b[0m \x1b[34m{}\x1b[0m\n",
                        rgb.0, rgb.1, rgb.2
                    );
                }

                // Печаю цвет в HEX формате
                _ => println!("Color in hex: \x1b[95m{}\x1b[0m\n", rand_hex()),
            }
        } else {
            println!("\x1b[31mНеверный формат ввода.\x1b[0m\n");
        };
    }
}

// Создаю цвет в RGB формате
fn rand_rgb() -> ColorRgb {
    ColorRgb(
        rng().random_range(1..=256),
        rng().random_range(1..=256),
        rng().random_range(1..=256),
    )
}

// Создаю цвет в HEX формате
fn rand_hex() -> String {
    let mut hex = String::from("#");
    let hex_mas = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];

    // Добавляю пока длинна строки не будут 7
    while hex.len() < 7 {
        let index = rng().random_range(0..=15);
        hex.push_str(hex_mas[index]);
    }

    hex
}
