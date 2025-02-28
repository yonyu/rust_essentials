mod vectors;
mod haspmaps;
mod challenge_count_words;
mod case_insensitive_sort;

// cargo run ".\from earth to moon.txt"
// cargo run -p collections "crates\collections\from earth to moon.txt"
// at the project root:
// `target\debug\collections.exe "..\..\crates\collections\from earth to moon.txt"`
fn main() {
    let path = match std::env::args().nth(1) { //"From Earth to Moon.txt".to_string()
        Some(path) => path,
        None => {
            println!("Program requires an argument: <file path>");
            std::process::exit(2);
        },
    };
    challenge_count_words::count_words(&path); //"From Earth to Moon.txt"
}
