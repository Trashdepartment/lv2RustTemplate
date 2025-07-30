use dsp::Voice;
use fundsp::numeric_array::NumericArray;
// Use the prelude and the `wmidi` crate.
use fundsp::prelude::*;
use lv2::prelude::Map;
use lv2::prelude::*;

use std::array;
use wmidi::*;
mod dsp;

#[derive(PortCollection)]
pub struct Ports {
    midi: InputPort<AtomPort>,
    left_output: OutputPort<Audio>,
    right_output: OutputPort<Audio>,
}
// Now, an additional host feature is needed. A feature is something that
// implements the `Feature` trait and usually wraps a certain functionality of
// the host; In this case mapping URIs to URIDs. The discovery and validation of
// features is done by the framework.
#[derive(FeatureCollection)]
pub struct Features<'a> {
    map: LV2Map<'a>,
}
// Retrieving URIDs from the host isn't guaranteed to be real-time safe or even
// fast. Therefore, all URIDs that may be needed should be retrieved when the
// plugin is instantiated. The `URIDCollection` trait makes this easy: It
// provides a single method that creates an instance of itself from the mapping
// feature, which can also be generated using this `derive` macro.
#[derive(URIDCollection)]
pub struct URIDs {
    atom: AtomURIDCollection,
    midi: MidiURIDCollection,
    unit: UnitURIDCollection,
}

#[uri("https://luojunxiang/lv2/{{cookiecutter.project_name}}/plugin")]
pub struct {{cookiecutter.plugin_name}} {
    active_voices: [bool; 128],
    notes: [Shared; 128],
    program: u8,
    urids: URIDs,
    voices: [Voice; 128], //Yes it is an AudioNode
}

impl {{cookiecutter.plugin_name}} {
    // A function to write a chunk of output, to be called from `run()`. If the gate
    // is high, then the input will be passed through for this chunk, otherwise
    // silence is written.
    fn on_note(&mut self, midi: usize) {
        if !self.active_voices[midi] {
            self.active_voices[midi] = true;
        }
        self.notes[midi].set(1.0);
    }
    fn off_note(&mut self, midi: u8) {
        self.notes[midi as usize].set(-1.0);
    }
    fn init_voices(&mut self, sample_rate: f64) {
        let array: [Voice; 128] =
            array::from_fn(|i| dsp::create_voice(i as u8, &self.notes[i], sample_rate));
        self.voices = array;
    }
    fn tick(&mut self, left_frame: &mut f32, right_frame: &mut f32) {
        let mut out_tick: NumericArray<f32, U2> = [0.0; 2].into();
        for i in 0..128 {
            match self.active_voices[i] {
                true => {
                    let voice = &mut self.voices[i];
                    let output = voice.tick(&Frame::from([]));
                    out_tick[0] += output[0];
                    out_tick[1] += output[1];
                }
                _ => continue,
            }
        }

        *left_frame = out_tick[0];
        *right_frame = out_tick[1];
    }
}

impl Plugin for {{cookiecutter.plugin_name}}  {
    type AudioFeatures = ();
    type InitFeatures = Features<'static>;
    type Ports = Ports;

    fn new(plugin_info: &PluginInfo, features: &mut Features<'static>) -> Option<Self> {
        let notes = array::from_fn(|_i| shared(-1.0));
        let mut result = Self {
            active_voices: [false; 128],
            notes,
            program: 0,
            urids: features.map.populate_collection()?,
            voices: array::from_fn(|_| dsp::create_default(plugin_info.sample_rate())),
        };
        result.init_voices(plugin_info.sample_rate());

        Some(result)
    }

    fn run(&mut self, ports: &mut Ports, _: &mut (), _: u32) {
        let mut offset: usize = 0;

        let mut midi_sequence = ports
            .midi
            .read(self.urids.atom.sequence, self.urids.unit.beat)
            .unwrap();
        for (left_out, right_out) in
            Iterator::zip(ports.left_output.iter_mut(), ports.right_output.iter_mut())
        {
            offset += 1;
            let (timestamp, message) = match midi_sequence.next() {
                Some((t, m)) => (t.as_frames(), m.read(self.urids.midi.wmidi, ())),
                _ => (None, None),
            };
            match (timestamp, message) {
                (Some(_t), Some(m)) => match m {
                    MidiMessage::NoteOn(_, note, _velocity) => self.on_note(note as usize),
                    MidiMessage::NoteOff(_, note, _) => self.off_note(note.into()),
                    _ => (),
                }, // Wtf is an idiomatic rust?
                _ => (),
            };
            self.tick(left_out, right_out);
        }
    }

    // During it's runtime, the host might decide to deactivate the plugin. When the
    // plugin is reactivated, the host calls this method which gives the plugin an
    // opportunity to reset it's internal state.
    fn activate(&mut self, _features: &mut Features<'static>) {}
}

lv2_descriptors!({{cookiecutter.plugin_name}});
