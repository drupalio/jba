use cpu;

pub struct Input {
    buttons: u8,
    directions: u8,
    joypad_sel: u8,
    col: Selected,
}

pub enum Button {
    A,
    B,
    Start,
    Select,
    Left,
    Up,
    Down,
    Right,
}

// Enum for which column of inputs is selected. See
// http://nocash.emubase.de/pandocs.htm#joypadinput for codes and what each
// column is.
#[deriving(FromPrimitive)]
pub enum Selected {
    Button = 0x20,
    Direction = 0x10,
    MltReq = 0x00,
}

impl Input {
    pub fn new() -> Input {
        Input {
            buttons: 0xf,
            directions: 0xf,
            col: Direction,
            joypad_sel: 0,
        }
    }

    pub fn reset(&mut self) {
        self.col = Direction;
        self.buttons = 0xf;
        self.directions = 0xf;
        self.joypad_sel = 0;
        // self.sgb.reset()
    }

    pub fn rb(&self, _addr: u16) -> u8 {
        match self.col {
            Button => self.buttons,
            Direction => self.directions,
            MltReq => 0xf - self.joypad_sel,
        }
    }

    pub fn wb(&mut self, _addr: u16, value: u8) {
        // The selected column is also negatively asserted, so invert the value
        // written in to get a positively asserted selection
        match FromPrimitive::from_u8(!value & 0x30) {
            Some(col) => { self.col = col; }
            None => {}
        }
        // self.sgb.receive((value >> 4) & 0x3);
    }

    // This is a mapping of key codes to the mask which will be AND'ed into the
    // correct value. These values are asserted low, so the relevant bit is
    // cleared. Here's what each bit position is:
    //
    // Bit 3 - P13 Input Down or Start (0=Pressed) 0111 = 0x7
    // Bit 2 - P12 Input Up or Select (0=Pressed) 1011 = 0xb
    // Bit 1 - P11 Input Left or Button B (0=Pressed) 1101 = 0xd
    // Bit 0 - P10 Input Right or Button A (0=Pressed) 1110 = 0xe

    pub fn keydown(&mut self, key: Button, if_: &mut u8) {
        *if_ |= cpu::IntJoypad as u8;
        match key {
            A      => { self.buttons &= 0xe; }
            B      => { self.buttons &= 0xd; }
            Start  => { self.buttons &= 0x7; }
            Select => { self.buttons &= 0xb; }
            Left   => { self.directions &= 0xd; }
            Up     => { self.directions &= 0xb; }
            Down   => { self.directions &= 0x7; }
            Right  => { self.directions &= 0xe; }
        }
    }

    pub fn keyup(&mut self, key: Button) {
        match key {
            A      => { self.buttons |= !0xe; }
            B      => { self.buttons |= !0xd; }
            Start  => { self.buttons |= !0x7; }
            Select => { self.buttons |= !0xb; }
            Left   => { self.directions |= !0xd; }
            Up     => { self.directions |= !0xb; }
            Down   => { self.directions |= !0x7; }
            Right  => { self.directions |= !0xe; }
        }
    }
}
