use sha1::Digest;
use::std::{
    env, // stdlib env library
    error::Error, // <- allow program to return any type with this trait
    fs::File, // allows files to be read/written to
    io::{BufRead, BufReader}, // 'reader' w internal buf and struct adding
};                            // buffering to said reader

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> { // "Box Errors"
    let args: Vec<String> = env::args().collect(); // collects vector of string objects.

    if args.len() != 3 { // check # of args
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim(); // setting hash_to_crack to provided hash arg
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH { // ensure valid hash
        return Err("sha1 hash is not valid".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() { // read through wordlist (no close method is needed in Rust becase RAII)
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack ==
            hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
                println!("Password found: {}", &common_password);
                return Ok(());
            } // compare provided hash to encoded wordlist hash
    }
    println!("password not found in wordlist *womp womp*");

    Ok(()) // function completed successfully but has no data to return 
}
