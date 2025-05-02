pub struct MemoryMap {
    pub rom_bank_00: [u8; 0x3FFF],
    pub rom_bank_01: [u8; 0x3FFF],
    pub vram: [u8; 0x1FFF],
    pub external_ram: [u8; 0x1FFF],
    pub wram: [u8; 0xFFF],
    pub wram_switch: [u8; 0xFFF],
    pub echo_ram: [u8; 0x1DFF], // PROHIBITED
    pub oam: [u8; 0x9F],
    pub io: [u8; 0x5F],
    pub hram: [u8; 0x7E],
    pub ie: u8,
}
impl MemoryMap {
    pub fn new() -> Self {
        Self {
            rom_bank_00: [0; 0x3FFF],
            rom_bank_01: [0; 0x3FFF],
            vram: [0; 0x1FFF],
            external_ram: [0; 0x1FFF],
            wram: [0; 0xFFF],
            wram_switch: [0; 0xFFF],
            echo_ram: [0; 0x1DFF],
            oam: [0; 0x9F],
            io: [0; 0x5F],
            hram: [0; 0x7E],
            ie: 0,
        }
    }
    fn read_rom_bank_00(&self, addr: u16) -> u8 {
        self.rom_bank_00[addr as usize]
    }
    pub fn write_rom_bank_00(&mut self, addr: u16, value: u8) {
        self.rom_bank_00[addr as usize] = value;
    }
    fn read_rom_bank_01(&self, addr: u16) -> u8 {
        self.rom_bank_01[addr as usize]
    }

    fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }
    fn read_external_ram(&self, addr: u16) -> u8 {
        self.external_ram[addr as usize]
    }
    fn read_wram(&self, addr: u16) -> u8 {
        self.wram[addr as usize]
    }
    fn read_wram_switch(&self, addr: u16) -> u8 {
        self.wram_switch[addr as usize]
    }
    pub fn read(&self, addr: u16) -> u8 {
        let result = match addr {
            0x0000..0x4000 => self.read_rom_bank_00(addr),
            0x4000..0x8000 => self.read_rom_bank_01(addr),
            0x8000..0xA000 => self.read_vram(addr),
            0xA000..0xC000 => self.read_external_ram(addr),
            0xC000..0xD000 => self.read_wram(addr),
            0xD000..0xE000 => self.read_wram_switch(addr),
            0xE000..0xFE00 => 7,
            0xFE00..0xFEA0 => 8,
            0xFEA0..0xFF00 => 9,
            0xFF00..0xFF80 => 10,
            0xFF80..0xFFFF => 11,
            0xFFFF => 12,
            _ => 0,
        };

        result
    }
}
