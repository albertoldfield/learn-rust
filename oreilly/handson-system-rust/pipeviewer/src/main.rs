use std::{
    env,
    io::{self, Read, Write},
};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let mut total_bytes = 0;

    let silent = env::var("PV_SILENT").unwrap_or_default().is_empty();

    loop {
        let mut buffer = [0; CHUNK_SIZE];

        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => {
                total_bytes += x;
                x
            }
            Err(_) => break,
        };

        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }

    if !silent {
        eprintln!("total_bytes: {}", total_bytes);
    }
}
