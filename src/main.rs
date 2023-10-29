use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom, Write};

fn decrypt_file(input_path: &str, output_path: &str) -> Result<(), Error> {
    let mut file = File::open(input_path)?;

    let mut header = vec![0; 23];
    file.seek(SeekFrom::Start(9))?;
    file.read_exact(&mut header)?;

    for i in header.iter_mut() {
        if *i == b'$' {
            *i = b' ';
        }
    }
    let mut new_header = Vec::<u8>::new();
    let mut index = 0;
    while index < header.len() {
        if header[index] == b'$' {
            header[index] = b' ';
        }
        if index < header.len() - 4 && &header[index..(index + 4)] == b"avc1" {
            index += 4;
            continue;
        }
        new_header.push(header[index]);
        index += 1;
    }

    let mut decrypted = File::create(output_path)?;
    decrypted.write_all(new_header.as_slice())?;

    let mut buffer = vec![0; 256 * 1024 * 1024];
    loop {
        let length = file.read(&mut buffer)?;
        if length == 0 {
            break;
        }
        decrypted.write(&buffer[..length])?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    decrypt_file("audio.m4s", "sound.mp4")?;

    Ok(())
}
