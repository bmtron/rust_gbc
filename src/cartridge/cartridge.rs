use std::fs::File;
use std::io::Read;

//pub fn load_cartridge(rom_path: &str) -> Result<(), String> {}

pub fn load_cartridge_test() -> Vec<u8> {
    let cart_path = "../../src/cartridge/PokemonGold.gbc"; // TODO: temp file location

    let file = File::open(cart_path).unwrap();
    let mut file_bytes = file.bytes();

    //println!("File Size: {}", &file_bytes.count());
    let mut count = 0;
    let mut nintendo_logo_vec: Vec<u8> = Vec::new();
    for num in file_bytes {
        if count > 259 && count < 308 {
            nintendo_logo_vec.push(num.unwrap());
        }
        count += 1;
    }

    nintendo_logo_vec
}

pub fn load_cartridge_test_2() {
    let cart_path = "../../src/cartridge/PokemonGold.gbc"; // TODO: temp file location

    let file = File::open(cart_path).unwrap();
    let mut file_bytes = file.bytes();

    let mut count = 0;
    let mut title_vec: Vec<u8> = Vec::new();

    let mut code: u8 = 0;
    let mut rom_size: u8 = 0;
    let mut ram_size: u8 = 0;
    let mut dest_code: u8 = 0;

    for num in file_bytes {
        let unwrapped_num = num.unwrap();

        if count >= 308 && count < 323 && unwrapped_num != 0 {
            //println!("{}", unwrapped_num);
            print!("{}", unwrapped_num as char);
        }
        if count == 327 {
            code = unwrapped_num;
        }
        if count == 328 {
            rom_size = unwrapped_num;
        }
        if count == 329 {
            ram_size = unwrapped_num;
        }
        if count == 330 {
            dest_code = unwrapped_num;
        }
        count += 1;
    }

    println!("");
    println!("CART_TYPE: {:X?}", code);
    println!("ROM_SIZE_UNALTERED: {}", rom_size);
    println!("ROM_SIZE: {}", 32 * (1 << rom_size));
    println!("RAM_SIZE: {}", ram_size);
    println!("DEST_CODE: {}", dest_code);
    println!("");
}
