use std::util;

use cpu::Cpu;
use input;
use mem::Memory;

pub struct Gb {
    priv cpu: Cpu,
    priv mem: Memory,
    priv fps: uint,
    priv cycles: uint,
}

impl Gb {
    pub fn new() -> Gb {
        let mut gb = Gb {
            mem: Memory::new(),
            cpu: Cpu::new(),
            fps: 0,
            cycles: 0,
        };
        gb.mem.power_on();
        return gb;
    }

    pub fn load(&mut self, rom: ~[u8]) {
        self.mem.load_cartridge(rom);
    }

    pub fn frame(&mut self) {
        // http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
        // for the timing for this constant
        self.cycles += 70224;

        while self.cycles <= 70224 {
            let time = self.cpu.exec(&mut self.mem);
            self.mem.timer.step(time, &mut self.mem.if_);
            self.mem.gpu.step(time, &mut self.mem.if_);
            self.cycles -= time;
        }

        self.fps += 1;
    }

    pub fn image<'a>(&'a self) -> &'a [u8] {
        self.mem.gpu.image_data.as_slice()
    }

    pub fn frames(&mut self) -> uint {
        util::replace(&mut self.fps, 0)
    }

    pub fn keydown(&mut self, key: input::Button) {
        self.mem.input.keydown(key, &mut self.mem.if_);
    }

    pub fn keyup(&mut self, key: input::Button) {
        self.mem.input.keyup(key);
    }
}