use muzik::comping_generator::ChordCompingGenerator;
use std::path::Path;

fn main() {
    let generator = ChordCompingGenerator::new(65, 70);
    // First A section of `Ruby, My Dear`,
    // from the amazing composer Thelonious Monk (October 10, 1917 – February 17, 1982 ❤️).
    let input2 = "|Fm9 L Bb13b9 L|Ebmaj7l.Fm7,F#m7,Gm7,Abm7,Am7,|Gm9L C13b9L|Fmaj7l Gm7l 
        Abm7l Am7l|Bbm7L Eb13b9L|Abmaj7lBbm7,Bm7,Cm7L|Bbmadd11LAadd9L|E7susLBb7b5L|";
    let smf = generator.from_string(input2, false);
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
}
