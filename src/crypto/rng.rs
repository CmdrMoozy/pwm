use byteorder::{LittleEndian, ReadBytesExt};
use rand::Rng;
use sodiumoxide::randombytes::randombytes;
use std::io::Cursor;

/// This structure implements the `Rng` trait from the `rand` crate using
/// `sodiumoxide`'s `randombytes` function. This implies that this random
/// number generator is both thread safe, and cryptographically secure (e.g.
/// suitable for generating key material or passwords).
pub struct Generator;

impl Rng for Generator {
    fn next_u32(&mut self) -> u32 {
        let bytes = randombytes(4);
        let mut rdr = Cursor::new(bytes);
        rdr.read_u32::<LittleEndian>().unwrap()
    }
}
