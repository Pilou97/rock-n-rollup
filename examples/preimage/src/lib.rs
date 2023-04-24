use rock_n_rollup::{
    core::{Application, Runtime},
    plugins::{dac::Dac, logger::Logger},
    services::external::External,
};

pub fn hello<R: Dac + Logger>(rt: &mut R, _: External<Vec<u8>>) {
    rt.log("First test");
    rt.log("First test2");

    let hash: [u8; 33] = [
        0x00, 0xD4, 0x97, 0x98, 0xB2, 0xE2, 0x3F, 0xF9, 0xF4, 0x86, 0x80, 0x79, 0x3A, 0x64, 0x9F,
        0xE7, 0xB7, 0x87, 0xDB, 0x5C, 0x56, 0x49, 0xAC, 0xF8, 0xFC, 0x1C, 0x95, 0x0C, 0xDA, 0x12,
        0xE3, 0xAC, 0x82,
    ];

    rt.log("Second test");

    let preimage = rt.read_from_dac(&hash).unwrap();

    let str = String::from_utf8(preimage).unwrap();

    rt.log(&str);
}

#[rock_n_rollup::main]
pub fn main<R: Runtime>(application: &mut Application<R>) {
    application.register(hello).run();
}
