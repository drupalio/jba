/**
 * Contains logic for the CPU of the GB.
 *
 * @constructor
 * @implements {Serializable}
 */
JBA.CPU = function() {
  this.registers = new Z80.Registers();
  this.reset();
};

/**
 * Interrupt codes for each interrupt the gameboy can receive
 *
 * @enum {number}
 */
JBA.INT = {
  VBLANK:  0x01,
  LCDSTAT: 0x02,
  TIMER:   0x04,
  SERIAL:  0x08,
  JOYPAD:  0x10
};

/* Table of what action to take on delivering an interrupt. This table is
   indexed based on the IF and IE flags AND'ed together. That index leads to
   a function which will deliver the necessary interrupt. */
JBA.CPU.Interrupts = [];

(function() {
  function deliver_interrupt(mask, rst) {
    return function(r, m, u8regs, u16regs) {
      r.ime = 0;
      r.halt = 0;
      m._if &= (~mask) & 0xff;
      rst(r, m, u8regs, u16regs);
    };
  }

  for (var i = 0; i < 32; i++) {
    if (i & JBA.INT.VBLANK) { /* vblank => INT 40 */
      JBA.CPU.Interrupts[i] = deliver_interrupt(JBA.INT.VBLANK, Z80.ops.rst_40);
    } else if (i & JBA.INT.LCDSTAT) { /* LCD STAT => INT 48 */
      JBA.CPU.Interrupts[i] = deliver_interrupt(JBA.INT.LCDSTAT,
                                                Z80.ops.rst_48);
    } else if (i & JBA.INT.TIMER) { /* timer => INT 50 */
      JBA.CPU.Interrupts[i] = deliver_interrupt(JBA.INT.TIMER, Z80.ops.rst_50);
    } else if (i & JBA.INT.SERIAL) { /* serial => INT 58 */
      JBA.CPU.Interrupts[i] = deliver_interrupt(JBA.INT.SERIAL, Z80.ops.rst_58);
    } else if (i & JBA.INT.JOYPAD) { /* joypad => INT 60 */
      JBA.CPU.Interrupts[i] = deliver_interrupt(JBA.INT.JOYPAD, Z80.ops.rst_60);
    } else { /* No interrupt to deliver */
      JBA.CPU.Interrupts[i] = function() {};
    }
  }
})();

JBA.CPU.prototype = {
  /** @type {JBA.Memory} */
  memory: null,

  ticks: 0,

  reset: function() {
    this.ticks = 0;
    this.registers.reset();
  },

  // Don't care about ticks, not sure about size anyway.
  serialize: function(io) { this.registers.serialize(io); },
  deserialize: function(io) { this.registers.deserialize(io); },

  /**
   * Exec one instruction for this CPU
   *
   * @return {number} the number of cycles the instruction took to run.
   */
  exec: function() {
    var r = this.registers, m = this.memory, u8 = r.u8regs, u16 = r.u16regs;

    /* When the CPU halts, it simply goes into a "low power mode" that doesn't
       execute any more instructions until an interrupt comes in. Deferring
       until this interrupt happens is fairly difficult, so we just don't
       execute any instructions. We simulate that the 'nop' instruction
       continuously happens until an interrupt comes in which will disable the
       halt flag */
    if (r.halt == 0) {
      var instruction = m.rb(u16[Z80.PC]++);
      Z80.map[instruction](r, m, u8, u16);
    } else {
      u8[Z80.M] = 1;
    }

    var ticks = u8[Z80.M] * 4;
    u8[Z80.M] = 0;

    // See http://nocash.emubase.de/pandocs.htm#interrupts
    if (r.ime) {
      var interrupts = m._if & m._ie;

      if (interrupts) {
        JBA.CPU.Interrupts[interrupts](r, m, u8, u16);

        ticks += u8[Z80.M] * 4;
      }
    }

    this.ticks += ticks;
    if (m.timer) {
      m.timer.step(ticks / 4);
    }

    return ticks;
  }
};
