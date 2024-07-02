use rand::Rng;
use std::io;

struct ColorRgb(u32, u32, u32);

fn main() {
    println!("Hello!");
    println!("Это генератор цветов.");
    loop {
        println!("Выберете формат :\n");

        println!("Введите 1 если в RGB");
        println!("Введите 2 если в HEX \n");

        println!("Введите 0 если хотите выйти");

        let mut format = String::new();
        io::stdin().read_line(&mut format).expect("Failed");

        let format: i32 = format.trim().parse().expect("Failed");

        if format == 0 {
            break;
        } else if format == 1 {
            let rand_color = ColorRgb(
                rand::thread_rng().gen_range(1..=256),
                rand::thread_rng().gen_range(1..=256),
                rand::thread_rng().gen_range(1..=256),
            );

            println!(
                "Color in rgb :  r {} g {} b {}\n",
                rand_color.0, rand_color.1, rand_color.2
            );
        } else {
            let hex = rand_hex();

            println!("Color in hex: {}\n", hex);
        }
    }
}

fn rand_hex() -> String {
    let mut hex = String::from("#");
    let hex_mas = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ];

    while hex.len() < 7 {
        let index = rand::thread_rng().gen_range(0..=15);
        hex.push_str(&hex_mas[index]);
    }

    hex
}
