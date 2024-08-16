use std::path::Path;

use midi_comping::comping_generator::ChordCompingGenerator;

fn main() {
    let generator = ChordCompingGenerator::new(120, 70);
    //  "|DbMaj7,..,}.Abm11,|}}.Db713b9l,|GbMaj7,.GbMaj7+,.Gb6,..Ebm7b511,|}}Ab713b9,..,|Gm7b511,.}C7#9b13,.}|Fm7b5add11l}Bb7b9b13l.,|Ebm11l}.,.Ab13b9,|}.,.,.,|"
    // "|Fm6Maj79,.}}Bbm69.l.}}}|Fm6.l.lGb13#11.l..F7#9b13l.}|Bbm69,.,.,.,.|Gm7b5}l.C7#9b13l.|FmMaj7add9,..,}}|Dm7b5.l.l}|Db13#11l}l}|C7#9b13,.l}}|FmMaj79l.Fmb69l.Fm69l|Fm7.,}C7b9b13l}|",
    let mut chords = vec![
        "DbMaj7",
        "Abm11",
        "Db713b9",
        "GbMaj7+",
        "Gb6",
        "Ebm7b511",
        "Ab713b9",
        "Gm7b511",
        "C7#9b13",
        "Fm7b5add11",
        "Bb7b9b13",
        "Ebm11",
        "Ab13b9",
    ];
    let input =
        "|*,. ., } .*,| } } .*l ,|*O*O*| l _.*,|*,. } *,. } |*l } *l .,|*l } ., .*,|} ., ., .,|";
    let smf = generator.with_wildcards(input, &mut chords, true);
    match smf {
        Ok(smf) => {
            let path = Path::new("comping").with_extension("mid");
            let mut file = std::fs::File::create(path).unwrap();
            smf.write_std(&mut file).unwrap();
        }
        Err(e) => {
            dbg!(e);
        }
    }
}
