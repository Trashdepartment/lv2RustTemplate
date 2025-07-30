use fundsp::prelude::*;
pub struct Voice(An<Unit<U0, U2>>);

impl Voice {
    pub fn tick(&mut self, input: &Frame<f32, U0>) -> Frame<f32, U2> {
        let result = self.0.tick(input);
        return result;
    }
}

pub fn create_voice(midi: u8, input: &Shared, sample_rate: f64) -> Voice {
    let midifreq = midi_hz(midi as f32);
    let my_synth = constant(midifreq) >> sine::<f64>();
    let mut an_voice = var(input) >> adsr_live(0.2, 0.1, 0.5, 0.8) * my_synth >> (pass() ^ pass());
    let mut type_erased = unit::<U0, U2>(Box::new(an_voice));
    type_erased.set_sample_rate(sample_rate);
    Voice(type_erased)
}
pub fn create_default(sample_rate: f64) -> Voice {
    let mut default = constant(0.0) >> (pass() ^ pass());
    let mut type_erased = unit::<U0, U2>(Box::new(default));
    type_erased.set_sample_rate(sample_rate);
    Voice(type_erased)
}
