# Dac

The `Dac` plugin gives you a way to read data from the reveal data channel (populated by the DAC).

```rust, noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::plugins::dac::*;

fn transition<R: Dac>(rt: &mut R) {
    let hash: PreimageHash = PreimageHash::try_from("00D49798B2E23FF9F48680793A649FE7B787DB5C5649ACF8FC1C950CDA12E3AC82").unwrap();

    let data: Vec<u8> = rt.read_from_dac(&hash).unwrap();
}
# fn main() {}
```
