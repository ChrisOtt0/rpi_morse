mod alphcon;
use std::io::{self, Write};

use alphcon::AlphCon;

fn main() {
    let port = 18;
    let unit = 250;
    let mut ac = AlphCon::new(port, unit);

    loop {
        let mut input = String::new();

        println!("Input text: "); 
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input
            .trim()
            .to_lowercase();
        
        ac.word_space();
        for c in input.chars() {
            if c == 'a' {
                ac.a();
            } else if c == 'b' {
                ac.b();
            } else if c == 'c' {
                ac.c();
            } else if c == 'd' {
                ac.d();
            } else if c == 'e' {
                ac.e();
            } else if c == 'f' {
                ac.f();
            } else if c == 'g' {
                ac.g();
            } else if c == 'h' {
                ac.h();
            } else if c == 'i' {
                ac.i();
            } else if c == 'j' {
                ac.j();
            } else if c == 'k' {
                ac.k();
            } else if c == 'l' {
                ac.l();
            } else if c == 'm' {
                ac.m();
            } else if c == 'n' {
                ac.n();
            } else if c == 'o' {
                ac.o();
            } else if c == 'p' {
                ac.p();
            } else if c == 'q' {
                ac.q();
            } else if c == 'r' {
                ac.r();
            } else if c == 's' {
                ac.s();
            } else if c == 't' {
                ac.t();
            } else if c == 'u' {
                ac.u();
            } else if c == 'v' {
                ac.v();
            } else if c == 'w' {
                ac.w();
            } else if c == 'x' {
                ac.x();
            } else if c == 'y' {
                ac.y();
            } else if c == 'z' {
                ac.z();
            } else if c == '1' {
                ac.one();
            } else if c == '2' {
                ac.two();
            } else if c == '3' {
                ac.three();
            } else if c == '4' {
                ac.four();
            } else if c == '5' {
                ac.five();
            } else if c == '6' {
                ac.six();
            } else if c == '7' {
                ac.seven();
            } else if c == '8' {
                ac.eight();
            } else if c == '9' {
                ac.nine();
            } else if c == '0' {
                ac.zero();
            } else if c == ' ' {
                ac.word_space();
            } 
             ac.letter_space();


































        }
    }
}
