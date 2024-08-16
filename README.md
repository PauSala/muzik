# Muzik

A library to generate chord compings in MIDI format

# Overview

muzik is a library for generating chord progressions as MIDI files.  
It allows you to generate compings from a string input that encompasses both chords and rhythm.

With this library, you can:  
**Use almost any chord**: Incorporate almost any chord type used in modern music.  
**Implement Voice Leading**: The library constructs progressions with a lead voice in mind, ensuring smooth transitions and a natural flow between chords.  
**Customize Rhythm**: Define simple rhythmic patterns.

The library's primary purpose is to enable the programmatic generation of compings from a relatively simple input string. Because of this focus, it has limitations compared to traditional score-writing software and is not intended to replace it.

# How it works

The rithm is defined as follows:

- `O` = whole note
- `_` = whole rest
- `L` = half note
- `~` = half rest
- `l` = quarter note
- `}` = quarter rest
- `,` = eigth note
- `.` = eight rest
- `|` represents a measure delimiter, it is supported for readability but can be omited.

The input is readed from left to right. When a chord is found it is set as the context for current rithm.  
For example (note that spaces are not mandatory, they are used to improve readability):

- `|Fm l l ~ |` A measure with an Fm played as two quarter notes and a half rest.
- `|Fm l l Bbm l .,|` A measure with an Fm played as two quarter and a Bbm played as a quarter, an eight rest and an eigth.

As seen in these examples, when a chord is placed in a mesure all the following notes are played as this chord until another chord is found.  
Note that as measure delimiters are not mandatory you can build rithms overflowing the measure, for example, assuming we are in 4/4:

- `|DbMaj7 l } } L } l l |` This template overflows the first measure representing a tie between two quarter notes.

In fact, the parser is agnostic to the time signature, so you could write chords and rithms with no measure delimiters at all.  
Once the string input is defined instantiate the [ChordCompingGenerator](comping_generator/struct.ChordCompingGenerator.html) and call its `from_string` method.

### Using wildcards

Another way to define the input is using wildcards for chords, so you could use this template:

- `|*l l ~ |*O   |*L *L |*l } l } |`

And then, use a vector of chords containing 5 chord strings (one for each wildcard `*`).
Then call the `from_wildcards` method with the input string and chord vector.  
We found this method useful when working with complex rithms and large chords (like AbMaj7#11add9)

# Examples

```rust
use muzik::comping_generator::ChordCompingGenerator;
use std::path::Path;

// Instantiate the generator with a bpm=65 and a lead note of 70 (a Bb3 MIDI code).
// A note around 68-74 should be a good choice.
let generator = ChordCompingGenerator::new(65, 70);

// First A section of `Ruby, My Dear`,
// from the amazing composer Thelonious Monk (October 10, 1917 – February 17, 1982 ❤️).
let rmd = "|Fm9 L Bb13b9 L|Ebmaj7l.Fm7,F#m7,Gm7,Abm7,Ab6,|Gm9L C13b9L|Fmaj7l Gm7l
       Abm7l Am7l|Bbm7L Eb13b9L|Abmaj7lBbm7,Bm7,Cm7L|Bbmadd11LAadd9L|E7susLBb7b5L|";
let smf = generator.from_string(rmd, false);
    match smf {
    Ok(smf) => {
        let path = Path::new("ruby_my_dear").with_extension("mid");
        let mut file = std::fs::File::create(path).unwrap();
        // Export the MIDI file fo disk
        smf.write_std(&mut file).unwrap();
    }
    Err(e) => {
        dbg!(e);
    }
}
```

# Limitations

- The rithm palette is narrowed in favor of simplicity. There are no sixteenth notes/rests, no triplets and no dots. This is fine for our use cases, but maybe in the future we will add at least sixteenths and dots.
- No swing feel. Since the MIDI files can be imported into any software that supports MIDI import, we leave the addition of swing functionality to those programs, which typically offer options to quantize MIDI with a swing feel.
