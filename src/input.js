/**
 * @constructor
 * @implements {Serializable}
 * @param {JBA.Memory} memory the memory which this input will set the interrupt
 *                     request flag for.
 */
JBA.Input = function(memory) {
  this.memory = memory;
  this.sgb    = new JBA.SGB(this);
  this.reset();
};

/**
 * Enum for which column of inputs is selected. See
 * http://nocash.emubase.de/pandocs.htm#joypadinput for codes and what each
 * column is.
 *
 * @enum {number}
 */
JBA.Input.SEL = {
  BUTTON: 0x20,
  DIRECTION: 0x10,
  MLT_REQ: 0x00
};

/**
 * This is a mapping of javascript key codes to the mask which will be AND'ed
 * into the correct value. These values are asserted low, so the relevant bit
 * is cleared. Here's what each bit position is:
 *
 *    Bit 3 - P13 Input Down  or Start    (0=Pressed) 0111 = 0x7
 *    Bit 2 - P12 Input Up    or Select   (0=Pressed) 1011 = 0xb
 *    Bit 1 - P11 Input Left  or Button B (0=Pressed) 1101 = 0xd
 *    Bit 0 - P10 Input Right or Button A (0=Pressed) 1110 = 0xe
 */
JBA.Input.Map = {
  buttons: {
    90: 0xe, // 'z' => button A
    88: 0xd, // 'x' => button B
    13: 0x7, // enter key => start
    188: 0xb // comma => select
  },

  directions: {
    37: 0xd, // left arrow => input left
    38: 0xb, // up arrow => input up
    39: 0xe, // right arrow => input right
    40: 0x7 // down arrow => input down
  }
};

JBA.Input.prototype = {
  /** @type {JBA.Input.SEL} */
  col: JBA.Input.SEL.DIRECTION,
  /** @type {JBA.Memory} */
  memory: null,
  /** @type {JBA.SGB} */
  sgb: null,

  /* These values are asserted LOW, so initialize them to all unasserted */
  buttons: 0xf,
  directions: 0xf,

  /**
   * Reset this Input device to its original state.
   */
  reset: function() {
    this.col = JBA.Input.SEL.DIRECTION;
    this.buttons    = 0xf;
    this.directions = 0xf;
    this.joypad_sel = 0;
    this.sgb.reset();
  },

  serialize: function(io) {
    io.wb(this.col);
    io.wb(this.buttons);
    io.wb(this.directions);
    this.sgb.serialize(io);
  },

  deserialize: function(io) {
    this.col = io.rb();
    this.buttons = io.rb();
    this.directions = io.rb();
    this.sgb.deserialize(io);
  },

  rb: function(addr) {
    switch (this.col) {
      case JBA.Input.SEL.BUTTON:    return this.buttons;
      case JBA.Input.SEL.DIRECTION: return this.directions;
      case JBA.Input.SEL.MLT_REQ:   return 0x0f - this.joypad_sel;
      default: return 0xf;
    }
  },

  wb: function(addr, value) {
    /* The selected column is also negatively asserted, so invert the value
       written in to get a positively asserted selection */
    this.col = ~value & 0x30;
    this.sgb.receive((value >> 4) & 0x3);
  },

  keydown: function(code) {
    var keep_propogating = true;
    var mask = JBA.Input.Map.directions[code];
    if (mask) {
      this.directions &= mask;
      this.memory._if |= JBA.INT.INPUT; /* Joypad interrupt bit */
      keep_propogating = false;
    }

    mask = JBA.Input.Map.buttons[code];
    if (mask) {
      this.buttons &= mask;
      this.memory._if |= JBA.INT.INPUT;
      keep_propogating = false;
    }

    return keep_propogating;
  },

  keyup: function(code) {
    var keep_propogating = true;
    var mask = JBA.Input.Map.directions[code];
    if (mask) {
      this.directions |= (~mask) & 0xf;
      keep_propogating = false;
    }

    mask = JBA.Input.Map.buttons[code];
    if (mask) {
      this.buttons |= (~mask) & 0xf;
      keep_propogating = false;
    }

    return keep_propogating;
  }
};
