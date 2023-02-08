use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Note {
    C = 0,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Note {
    pub fn get_first_of(&self) -> f32 {
        match self {
            Note::C => 16.35,
            Note::CSharp => 17.32,
            Note::D => 18.35,
            Note::DSharp => 19.45,
            Note::E => 20.60,
            Note::F => 21.83,
            Note::FSharp => 23.12,
            Note::G => 24.50,
            Note::GSharp => 25.96,
            Note::A => 27.50,
            Note::ASharp => 29.14,
            Note::B => 30.87
        }
    }

    pub fn octave(&self, octave: i32) -> f32 {
        self.get_first_of() * 2_f32.powi(octave)
    }
}


pub fn play_note(note: Note, octave: i32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let freq = note.octave(5);

    let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(3.0)).amplify(1.0);
    sink.append(source);
    sink.sleep_until_end();    
}
