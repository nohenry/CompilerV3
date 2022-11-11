#![no_main]
#![no_std]

trait Write {
    #[no_mangle]
    fn write(&self) {}
}

struct Data {}

impl Write for Data {}

#[no_mangle]
fn af(b: &dyn Write) {
    b.write();
}

#[no_mangle]
extern "C" fn main() {
    let data = Data {};
    af(&data);
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {

    }
}