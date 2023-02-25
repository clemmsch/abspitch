mod notes;
use std::{io, time::Duration};
use rand::prelude::*;

use crate::notes::{Note, play_note};



fn main() {
    loop {
        println!("Enter a command: ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        if stdin.read_line(&mut buffer).is_ok() {
            match buffer.as_str().trim_end() {
                "eq" => { are_equal() }
                "which" => { which() }
                "is_note" => { is_note() }
                "ten_of" => { ten_of() }
                _ => { println!("Unknown command '{}'.", buffer.trim_end())}
            }
        }    
    }
}

pub fn ten_of() {
    let note = {
        println!("Which note do you want to listen to for?:");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        let e = Note::try_from(buffer.trim_end());
        if let Ok(note) = e {
            note
        } else {            
            println!("Invalid note: '{}'", buffer.trim_end());
            return;
        }
    };
    for i in 0..10 {
        println!("Playing {:?}{}", note, i);
        play_note(note, i);
        std::thread::sleep(Duration::from_millis(200));
    }

}

pub fn which() {}

pub fn is_note() {
    let note = {
        println!("Which note do you want to listen out for?:");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        let e = Note::try_from(buffer.trim_end());
        if let Ok(note) = e {
            note
        } else {            
            println!("Invalid note: '{}'", buffer.trim_end());
            return;
        }
    };


    let mut correct = 0;
    let mut offness = 0;
    let mut incorrect = 0;
    for _ in 0..10 {
        let mut rng = thread_rng();
        let note_a: Note = unsafe { std::mem::transmute((rng.gen_range(0_u32..11_u32))) }; 
        let octave_a = rng.gen_range(3..8);

        let same = note_a == note;

        play_note(note_a, octave_a);
        println!("Was this a {:?} (e = equal, n = not equal)", note);
        let mut buffer = String::new();
        let stdin = io::stdin();
        if stdin.read_line(&mut buffer).is_ok() {
            // Trim end.
            let trimmed = buffer.trim_end();
            let answer = trimmed == "e";
            if answer == same {
                correct += 1;
                println!("You were correct: The first note was {:?}{}, the original note was {:?}. Your answer: {}. Correct answer {}.", note_a, octave_a, note, same, answer);
            } else {
                incorrect += 1;
                offness += (note_a as i32 - note as i32).abs() as u32;
                println!("You were incorrect: The first note was {:?}{}, the original note was {:?}. Your answer: {}. Correct answer {}.", note_a, octave_a, note, same, answer);
            }
            buffer.clear();
        }
    }
    println!("Correct: {}; Incorrect {}, avg. offness: {}, percentage correct: {}", correct, incorrect, offness / incorrect, correct / incorrect);
}


pub fn are_equal() {
    let mut correct = 0;
    let mut offness = 0;
    let mut incorrect = 0;
    for _ in 0..10 {
        let mut rng = thread_rng();
        let note_a: Note = unsafe { std::mem::transmute((rng.gen_range(0_u32..11_u32))) }; 
        let octave_a = rng.gen_range(3..6);

        let same: bool = rng.gen();
        let note_b = if same { note_a } else { 
            let mut gen = rng.gen_range(0_u32..11_u32);
            if gen == (note_a as u32) { gen += 1 }  
            unsafe { std::mem::transmute(gen) }
        };
        let mut octave_b = rng.gen_range(3..6);
        if octave_b == octave_a {  octave_b += 1; }

        play_note(note_a, octave_a);
        std::thread::sleep(Duration::from_millis(500));
        play_note(note_b, octave_b);

        let mut buffer = String::new();
        let stdin = io::stdin();
        if stdin.read_line(&mut buffer).is_ok() {
            // Trim end.
            let trimmed = buffer.trim_end();
            let answer = trimmed == "e";
            if answer == same {
                correct += 1;
                println!("You were correct: The first note was {:?}{}, the second note was {:?}{}. Your answer: {}. Correct answer {}.", note_a, octave_a, note_b, octave_b, same, answer);
            } else {
                incorrect += 1;
                offness += (note_a as i32 - note_b as i32).abs() as u32;
                println!("You were incorrect: The first note was {:?}{}, the second note was {:?}{}. Your answer: {}. Correct answer {}.", note_a, octave_a, note_b, octave_b, same, answer);
            }
            buffer.clear();
        }
    }
    println!("Correct: {}; Incorrect {}, avg. offness: {}, percentage correct: {}", correct, incorrect, offness / incorrect, correct / incorrect);

}