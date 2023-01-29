mod alphcon;
use alphcon::AlphCon;

fn main() {
    let port = 18;
    let unit = 250;
    let mut ac = AlphCon::new(port, unit);

    ac.word_space();
    ac.a();
    ac.letter_space();
    ac.b();
    ac.letter_space();
    ac.c();
    ac.letter_space();
}
