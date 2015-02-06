#![feature(path, core, std_misc, os, io, env)]
#![cfg_attr(test, feature(hash))]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(test, allow(dead_code))]

extern crate time;
extern crate getopts;
#[macro_use] extern crate log;
extern crate env_logger;

use std::env;
use std::old_io::File;

macro_rules! dpanic( ($($e:tt)*) => ({
    if cfg!(not(ndebug)) {
        panic!($($e)*);
    }
}) );

mod cpu;
mod gb;
mod gpu;
mod input;
mod mem;
mod rtc;
mod sgb;
mod timer;
#[cfg(test)] mod tests;

#[path = "gl.rs"] mod app;

fn usage(opts: &getopts::Options) {
    let prog = env::args().next().unwrap().into_string().unwrap();
    println!("{}", opts.usage(&format!("usage: {} [options] <rom>", prog)));
}

fn main() {
    env_logger::init().unwrap();
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "show this message")
        .optflag("", "fps", "don't run a display, just print FPS")
        .optopt("g", "gb", "type of gameboy to run", "[gb|cgb|sgb]");
    let matches = match opts.parse(env::args().skip(1)) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };
    if matches.opt_present("h") || matches.opt_present("help") ||
       matches.free.len() == 0 {
        return usage(&opts);
    }

    let rom = File::open(&Path::new(matches.free[0].as_slice())).read_to_end();
    let rom = match rom {
        Ok(rom) => rom,
        Err(e) => {
            println!("failed to read {}: {}", matches.free[0].as_slice(), e);
            return
        }
    };

    let mut gb = gb::Gb::new(match matches.opt_str("gb").as_ref().map(|s| s.as_slice()) {
        Some("gb") => gb::GameBoy,
        Some("cgb") => gb::GameBoyColor,
        Some("sgb") => gb::SuperGameBoy,
        Some(s) => {
            println!("Invalid gameboy type: {}", s);
            println!("Supported types: gb, cgb, sgb");
            return usage(&opts);
        }
        None => {
            match mem::Memory::guess_target(rom.as_slice()) {
                Some(target) => target,
                None => gb::GameBoyColor,
            }
        }
    });
    gb.load(rom);

    // TODO: needs native timers
    if matches.opt_present("fps") {
        let mut last = time::precise_time_ns();
        loop {
            gb.frame();
            let cur = time::precise_time_ns();
            if cur - last >= 1000000000 {
                println!("{}", gb.frames());
                last = cur;
            }
        }
    }

    app::run(gb);
}
