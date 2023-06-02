use crate::bios::Bios;


pub struct Interconnect {
    bios: Bios,
}

impl Interconnect {
    pub fn new(bios: Bios) -> Interconnect {
        Interconnect { bios }
    }

    pub fn load32(&self, addr: u32) -> u32 {
        if let Some(offset) = map::BIOS.contains(addr) {
            return  self.bios.load32(offset);
        }

        panic!("unhandled fetch32 at addr {:08x}", addr);
    }
}

mod map {
    pub struct Range(u32, u32);

    impl Range {
        pub fn contains(self, addr: u32) -> Option<u32> {
            let Range(start, lenght) = self;

            if addr >= start && addr <= start + lenght {
                Some(addr - start)
            }
            else {
                None
            }
        }
    }

    pub const BIOS: Range = Range(0xbfc00000, 512 * 1024);
}