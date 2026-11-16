#![no_std]
#![no_main]

use panic_halt as _;

/*

Currently getting onto the board with:

cargo build --release
rust-objcopy -O binary target/thumbv7em-none-eabihf/release/feather-experiments flash.bin
../uf2/utils/uf2conv.py flash.bin -c -b 0x26000 -f 0xADA52840
cp flash.uf2 /run/media/kevan/FTHR840BOOT/

TODO: maybe automate the process a bit more
- how?

*/

#[cortex_m_rt::entry]
fn main() -> ! {
    // Go into bootloader on next warm reset.
    // See `main.c` in https://github.com/adafruit/Adafruit_nRF52_Bootloader
    write(0x2000_7F7C, 0x5A1AD5); // SALADS

    loop {}
}

fn write(addr: u32, value: u32) {
    unsafe { (addr as *mut u32).write_volatile(value) };
}
