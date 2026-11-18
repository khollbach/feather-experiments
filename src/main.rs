#![no_std]
#![no_main]

use cortex_m::asm;
use defmt_rtt as _;
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("hi");
    defmt::panic!("bye");

    // Got these values from `src/boards/feather_nrf52840_express/board.h` in
    // https://github.com/adafruit/Adafruit_nRF52_Bootloader
    led_init(1, 15); // port 1, pin 15 -- red led
    led_init(1, 10); // port 1, pin 10 -- blue led

    let mut state = true;

    loop {
        if state {
            led_on(1, 15);
            led_on(1, 10);
        } else {
            led_off(1, 15);
            led_off(1, 10);
        }

        busy_wait(1_000_000);

        state ^= true;
    }
}

fn led_init(port: u32, pin: u32) {
    assert!(port < 2);
    assert!(pin < 32);

    let pin_cnf = 0x5000_0000 + 0x300 * port + 0x700 + 0x4 * pin;
    let cnf = 0x0000_0003; // configure as output
    write(pin_cnf, cnf);
}

fn led_on(port: u32, pin: u32) {
    assert!(port < 2);
    assert!(pin < 32);

    let out_set = 0x5000_0000 + 0x300 * port + 0x508;
    write(out_set, 1 << pin);
}

fn led_off(port: u32, pin: u32) {
    assert!(port < 2);
    assert!(pin < 32);

    let out_clr = 0x5000_0000 + 0x300 * port + 0x50c;
    write(out_clr, 1 << pin);
}

fn write(addr: u32, value: u32) {
    unsafe { (addr as *mut u32).write_volatile(value) };
}

fn busy_wait(num_loops: u32) {
    for _ in 0..num_loops {
        asm::nop();
    }
}
