mod notes;
use std::{io, thread::sleep_ms, time::Duration};
use rand::prelude::*;

use crate::notes::{Note, play_note};



fn main() {
    loop {
        let mut rng = thread_rng();
        let note_a: Note = unsafe { std::mem::transmute((rng.gen_range(0_u32..11_u32))) }; 
        let octave_a = rng.gen_range(2..8);

        let same: bool = rng.gen();
        let note_b = if same { note_a } else { 
            let mut gen = rng.gen_range(0_u32..11_u32);
            if gen == (note_a as u32) { gen += 1 }  
            unsafe { std::mem::transmute(gen) }
        };
        let mut octave_b = rng.gen_range(2..8);
        if octave_b == octave_a { octave_b += 1; }

        play_note(note_a, octave_a);
        std::thread::sleep(Duration::from_millis(500));
        play_note(note_b, octave_b);

        let mut buffer = String::new();
        let stdin = io::stdin();
        if stdin.read_line(&mut buffer).is_ok() {
            // Trim end.
            let trimmed = buffer.trim_end();
            println!("You typed: [{trimmed}]");
            let answer = trimmed == "e";
            if answer == same {
                println!("You were correct: The first note was {:?}{}, the second note was {:?}{}. Your answer: {}. Correct answer {}.", note_a, octave_a, note_b, octave_b, same, answer);
            } else {
                println!("You were incorrect: The first note was {:?}{}, the second note was {:?}{}. Your answer: {}. Correct answer {}.", note_a, octave_a, note_b, octave_b, same, answer);
            }
            buffer.clear();
        }
        println!("lmao");
    }
}