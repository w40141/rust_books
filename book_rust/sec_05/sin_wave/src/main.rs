mod mics;
mod mml;

use anyhow::Result;

fn main() -> Result<()> {
    // lib::a(3)
    // mics::chocho()
    //
    // kaeru no uta
    let mml = format!(
        "{}{}",
        "o5 l4 cdef edl2c l4 efga gfl2e", "l4 crcr crcr l8 ccdd eeff l4 ed l2c"
    );
    let notes = mml::parse(mml);
    let bpm = 120.0;
    mml::write("kaeru.wav", notes, bpm);
    Ok(())
}
