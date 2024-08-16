use midi_comping::parser::parse::ChordCompingGenerator;

fn main() {
    // |,..,__|,.\.,_|,._,..,|__.,_|
    let generator = ChordCompingGenerator::new();
    //  "|DbMaj7,..,_.Abm11,|__.Db713b9\\,|GbMaj7,.GbMaj7+,.Gb6,..Ebm7b511,|__Ab713b9,..,|Gm7b511,._C7#9b13,._|Fm7b5add11\\_Bb7b9b13\\.,|Ebm11\\_.,.Ab13b9,|_.,.,.,|"
    // "|Fm6Maj79,.__Bbm69.\\.___|Fm6.\\.\\Gb13#11.\\..F7#9b13\\._|Bbm69,.,.,.,.|Gm7b5_\\.C7#9b13\\.|FmMaj7add9,..,__|Dm7b5.\\.\\_|Db13#11\\_\\_|C7#9b13,.\\__|FmMaj79\\.Fmb69\\.Fm69\\|Fm7.,_C7b9b13\\_|",
    generator.scan("||", 120)
}
