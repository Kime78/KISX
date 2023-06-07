use crate::bios::Bios;

pub struct Interconnect {
    bios: Bios,
}

impl Interconnect {
    pub fn new(bios: Bios) -> Interconnect {
        Interconnect { bios }
    }

    pub fn load32(&self, addr: u32) -> u32 {
        if addr % 4 != 0 {
            panic!("Unaligned load32 addr {:08x}", addr);
        }

        if let Some(offset) = map::BIOS.contains(addr) {
            return self.bios.load32(offset);
        }

        panic!("unhandled fetch32 at addr {:08x}", addr);
    }

    pub fn store32(&self, addr: u32, val: u32) {
        if addr % 4 != 0 {
            panic!("Unaligned store32 addr {:08x}", addr);
        }

        if let Some(offset) = map::MEMCONTROL.contains(addr) {
            match offset {
                0 => {
                    if val != 0x1f000000 {
                        panic!("Bad expansion 1 base addres 0x{:08x}", val);
                    }
                }
                4 => {
                    if val != 0x1f802000 {
                        panic!("Bad expansion 2 base addres 0x{:08x}", val);
                    }
                }
                _ => println!("Unhandled write to MEMCONSTROL register"),
            }
        }
    }
}

mod map {
    pub struct Range(u32, u32);

    impl Range {
        pub fn contains(self, addr: u32) -> Option<u32> {
            let Range(start, lenght) = self;

            if addr >= start && addr <= start + lenght {
                Some(addr - start)
            } else {
                None
            }
        }
    }

    pub const BIOS: Range = Range(0xbfc00000, 512 * 1024);
    pub const MEMCONTROL: Range = Range(0x1f801000, 36);
}
