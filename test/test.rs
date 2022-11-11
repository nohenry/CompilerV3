#![no_main]
#![no_std]

trait Write {
    fn write(&self) {}
}

struct Data {}

impl Write for Data {
    fn write(&self) {}
}

fn af(b: &dyn Write) {
    b.write();
}

fn main() {
    let data = Data {};
    af(&data);
}
