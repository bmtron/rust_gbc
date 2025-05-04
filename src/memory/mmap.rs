#[derive(Debug)]
pub enum IllegalAccessError {
    IllegalReadAccessAttempt(u16),
    IllegalWriteAccessAttempt(u16),
}

impl std::fmt::Display for IllegalAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IllegalAccessError::IllegalReadAccessAttempt(num) => write!(
                f,
                "Illegal Attempt to Read Invalid Memory Address: Addr {}",
                num,
            ),
            IllegalAccessError::IllegalWriteAccessAttempt(num) => write!(
                f,
                "Illegal Attempt to Write Invalid Memory Address: Addr {}",
                num
            ),
        }
    }
}

impl std::error::Error for IllegalAccessError {}

pub struct MemoryMap {
    pub rom_bank_00: [u8; 0x4000],
    pub rom_bank_01: [u8; 0x4000],
    pub vram: [u8; 0x2000],
    pub external_ram: [u8; 0x2000],
    pub wram: [u8; 0x1000],
    pub wram_switch: [u8; 0x1000],
    pub echo_ram: [u8; 0x1E00], // PROHIBITED
    pub oam: [u8; 0xA0],
    pub io: [u8; 0x80],
    pub hram: [u8; 0x7E],
    pub ie: u8,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self {
            rom_bank_00: [0; 0x4000],
            rom_bank_01: [0; 0x4000],
            vram: [0; 0x2000],
            external_ram: [0; 0x2000],
            wram: [0; 0x1000],
            wram_switch: [0; 0x1000],
            echo_ram: [0; 0x1E00],
            oam: [0; 0xA0],
            io: [0; 0x80],
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
    fn write_rom_bank_01(&mut self, addr: u16, value: u8) {
        self.rom_bank_01[addr as usize] = value;
    }
    fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }
    fn write_vram(&mut self, addr: u16, value: u8) {
        self.vram[addr as usize] = value;
    }
    fn read_external_ram(&self, addr: u16) -> u8 {
        self.external_ram[addr as usize]
    }
    fn write_external_ram(&mut self, addr: u16, value: u8) {
        self.external_ram[addr as usize] = value;
    }
    fn read_wram(&self, addr: u16) -> u8 {
        self.wram[addr as usize]
    }
    fn write_wram(&mut self, addr: u16, value: u8) {
        self.wram[addr as usize] = value;
    }
    fn read_wram_switch(&self, addr: u16) -> u8 {
        self.wram_switch[addr as usize]
    }
    fn write_wram_switch(&mut self, addr: u16, value: u8) {
        self.wram_switch[addr as usize] = value;
    }
    fn read_oam(&self, addr: u16) -> u8 {
        self.oam[addr as usize]
    }
    fn write_oam(&mut self, addr: u16, value: u8) {
        self.oam[addr as usize] = value;
    }
    fn read_io(&self, addr: u16) -> u8 {
        self.io[addr as usize]
    }
    fn write_io(&mut self, addr: u16, value: u8) {
        self.io[addr as usize] = value;
    }
    fn read_hram(&self, addr: u16) -> u8 {
        self.hram[addr as usize]
    }
    fn write_hram(&mut self, addr: u16, value: u8) {
        self.hram[addr as usize] = value;
    }
    fn read_ie(&self) -> u8 {
        self.ie
    }
    fn write_ie(&mut self, value: u8) {
        self.ie = value;
    }

    pub fn read(&self, addr: u16) -> Result<u8, IllegalAccessError> {
        let result = match addr {
            0x0000..0x4000 => Ok(self.read_rom_bank_00(addr)),
            0x4000..0x8000 => Ok(self.read_rom_bank_01(addr)),
            0x8000..0xA000 => Ok(self.read_vram(addr)),
            0xA000..0xC000 => Ok(self.read_external_ram(addr)),
            0xC000..0xD000 => Ok(self.read_wram(addr)),
            0xD000..0xE000 => Ok(self.read_wram_switch(addr)),
            0xE000..0xFE00 => Err(IllegalAccessError::IllegalReadAccessAttempt(addr)), // EHCO RAM - PROHIBITED
            0xFE00..0xFEA0 => Ok(self.read_oam(addr)),
            0xFEA0..0xFF00 => Err(IllegalAccessError::IllegalReadAccessAttempt(addr)), // NOT USABLE - PROHIBITED
            0xFF00..0xFF80 => Ok(self.read_io(addr)),
            0xFF80..0xFFFF => Ok(self.read_hram(addr)),
            0xFFFF => Ok(self.read_ie()),
            _ => Err(IllegalAccessError::IllegalReadAccessAttempt(addr)),
        };

        result
    }
    pub fn write(&mut self, addr: u16, value: u8) -> Result<(), IllegalAccessError> {
        match addr {
            0x0000..0x4000 => Ok(self.write_rom_bank_00(addr, value)),
            0x4000..0x8000 => Ok(self.write_rom_bank_01(addr, value)),
            0x8000..0xA000 => Ok(self.write_vram(addr, value)),
            0xA000..0xC000 => Ok(self.write_external_ram(addr, value)),
            0xC000..0xD000 => Ok(self.write_wram(addr, value)),
            0xD000..0xE000 => Ok(self.write_wram_switch(addr, value)),
            0xE000..0xFE00 => Err(IllegalAccessError::IllegalWriteAccessAttempt(addr)), // EHCO RAM - PROHIBITED
            0xFE00..0xFEA0 => Ok(self.write_oam(addr, value)),
            0xFEA0..0xFF00 => Err(IllegalAccessError::IllegalWriteAccessAttempt(addr)),
            0xFF00..0xFF80 => Ok(self.write_io(addr, value)),
            0xFF80..0xFFFF => Ok(self.write_hram(addr, value)),
            0xFFFF => Ok(self.write_ie(value)),
            _ => Err(IllegalAccessError::IllegalWriteAccessAttempt(addr)),
        }
    }
}
