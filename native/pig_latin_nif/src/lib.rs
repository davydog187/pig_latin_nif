use rustler::{Encoder, Env, Error, Term};
use rustler::types::atom::ok;

rustler::rustler_export_nifs! {
    "Elixir.PigLatinNIF",
    [
        ("translate", 1, translate)
    ],
    None
}

fn translate_to_pig_latin(mut word: String) -> String {
    let mut tail = word.split_off(1);

    // TODO handle single letter words

    tail.push_str(" ");
    tail.push_str(&word);
    tail.push_str("ay");

    tail
}

fn translate<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let word: String = args[0].decode()?;

    // TODO handle sentences
    // TODO handle grammar

    Ok((ok(), translate_to_pig_latin(word)).encode(env))
}
