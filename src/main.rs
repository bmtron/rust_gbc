use cartridge::cartridge::{load_cartridge_test, load_cartridge_test_2};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat, PixelFormatEnum},
    render::{Texture, TextureAccess},
};
use std::time::Duration;

mod cartridge;
mod memory;

use memory::mmap::MemoryMap;

fn main() {
    load_cartridge_test_2();
    build_window(load_cartridge_test());
    let mut mmap: MemoryMap = MemoryMap::new();
    mmap.write_rom_bank_00(0x0400, 50);
    let val = mmap.read(0x0400);
    println!("val: {}", val.unwrap());
}

fn build_window(logo_vec: Vec<u8>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 160, 144);

    // 'running: loop {
    //     i = (i + 1) % 255;
    //     canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
    //     canvas.clear();
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //     }
    //     canvas.present();
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }
}
