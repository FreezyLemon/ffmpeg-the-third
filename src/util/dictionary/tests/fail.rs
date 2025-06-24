use ffmpeg_the_third::dictionary::*;

fn main() {}

pub fn drop_owned_while_ref_alive() {
    let dict = Dictionary::new();
    let dict_ref = dict.as_ref();
    drop(dict);
    println!("len: {}", dict_ref.len());
}

pub fn drop_owned_while_mut_alive() {
    let mut dict = Dictionary::new();
    let dict_mut = dict.as_mut();
    drop(dict);
    println!("len: {}", dict_mut.len());
}
