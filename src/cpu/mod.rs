//! Contains logic for the CPU of the GB.

use std::num::Int;

use gb;
use mem;

mod z80;

pub struct Cpu {
    regs: z80::Registers,
    ticks: uint,
}

#[allow(dead_code)]
pub enum Interrupts {
    IntVblank  = 0x01,
    IntLCDStat = 0x02,
    IntTimer   = 0x04,
    IntSerial  = 0x08,
    IntJoypad  = 0x10,
}

impl Cpu {
    pub fn new(target: gb::Target) -> Cpu {
        Cpu { regs: z80::Registers::new(target), ticks: 0 }
    }

    pub fn exec(&mut self, mem: &mut mem::Memory) -> uint {
        self.regs.int_step();

        // When the CPU halts, it simply goes into a "low power mode" that
        // doesn't execute any more instructions until an interrupt comes in.
        // Deferring until this interrupt happens is fairly difficult, so we
        // just don't execute any instructions. We simulate that the 'nop'
        // instruction continuously happens until an interrupt comes in which
        // will disable the halt flag
        let mut ticks = if self.regs.halt == 0 && self.regs.stop == 0 {
            let instruction = mem.rb(self.regs.bump());
            z80::exec(instruction, &mut self.regs, mem)
        } else {
            if self.regs.stop != 0 && mem.speedswitch {
                mem.switch_speeds();
                self.regs.stop = 0;
            }
            1
        };

        // See http://nocash.emubase.de/pandocs.htm#interrupts
        if self.regs.ime != 0 || self.regs.halt != 0 {
            let ints = mem.if_ & mem.ie_;

            if ints != 0 {
                let i = ints.trailing_zeros();
                if self.regs.ime != 0 {
                    mem.if_ &= !(1 << (i as uint));
                }
                self.regs.ime = 0;
                self.regs.halt = 0;
                self.regs.stop = 0;
                match i {
                    0 => { self.regs.rst(0x40, mem); }
                    1 => { self.regs.rst(0x48, mem); }
                    2 => { self.regs.rst(0x50, mem); }
                    3 => { self.regs.rst(0x58, mem); }
                    4 => { self.regs.rst(0x60, mem); }
                    _ => { dpanic!(); }
                }
                ticks += 1;
            }
        }

        match mem.speed {
            mem::Normal => { ticks *= 4; }
            mem::Double => { ticks *= 2; }
        }
        self.ticks += ticks;
        return ticks;
    }

    #[cfg(test)]
    pub fn is_loopback(&self, m: &mem::Memory) -> bool {
        self.regs.is_loopback(m)
    }
}
