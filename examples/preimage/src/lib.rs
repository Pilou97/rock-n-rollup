use rock_n_rollup::{
    core::{Application, Runtime},
    plugins::{
        dac::{Dac, PreimageHash},
        logger::Logger,
    },
    services::external::External,
};

pub fn hello<R: Dac + Logger>(rt: &mut R, _: External<Vec<u8>>) {
    rt.log("First test");
    rt.log("First test2");

    let hash = PreimageHash::try_from(
        "00D49798B2E23FF9F48680793A649FE7B787DB5C5649ACF8FC1C950CDA12E3AC82",
    )
    .unwrap();

    rt.log("Second test");

    let preimage = rt.read_from_dac(&hash).unwrap();

    let str = String::from_utf8(preimage).unwrap();

    rt.log(&str);
}

#[rock_n_rollup::main]
pub fn main<R: Runtime>(application: &mut Application<R>) {
    application.register(hello).run();
}
