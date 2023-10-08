use std::env;
use std::io::Read;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let path = env::args().nth(1).expect("no file given");

    let mut f = std::fs::File::open(path)?;
    let mut position = 0;
    let mut buffer = [0; BYTES_PER_LINE];
    while f.read_exact(&mut buffer).is_ok() {
        print!("[{:08x}]", position);

        for c in buffer {
            print!("{:02x} ", c);
        }

        if buffer.len() < BYTES_PER_LINE {
            for _ in 0..(BYTES_PER_LINE - buffer.len()) {
                print!("   ");
            }
        }

        for c in buffer {
            if (0x20..=0x7c).contains(&c) {
                print!("{}", char::from(c));
            } else {
                print!(".");
            }
        }

        println!();
        position += BYTES_PER_LINE;
    }

    Ok(())
}
