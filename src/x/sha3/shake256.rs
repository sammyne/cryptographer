use std::io::{self, Read, Write};

use vendored_sha3::digest::{ExtendableOutput, Input};
use vendored_sha3::{Sha3XofReader, Shake256};

#[derive(Clone)]
enum State {
    Absorbing(Shake256),
    Reading(Sha3XofReader),
}

/// SHAKE256 is the 256-bit SHAKE variable-output-length hash functions defined by FIPS-202
#[derive(Clone)]
pub struct SHAKE256 {
    state: State,
}

impl Read for SHAKE256 {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let State::Absorbing(v) = &self.state {
            // clone is inefficient
            self.state = State::Reading(v.clone().xof_result());
        }

        match self.state {
            State::Reading(ref mut v) => v.read(buf),
            _ => panic!("unexpected state"),
        }
    }
}

impl Write for SHAKE256 {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.state {
            State::Absorbing(ref mut v) => v.input(buf),
            State::Reading(_) => panic!("absorbing after reading"),
        }

        Ok(buf.len())
    }
}

impl super::ShakeHash for SHAKE256 {
    fn reset(&mut self) {
        self.state = State::Absorbing(Shake256::default());
    }
}

/// new_shake256 creates a new SHAKE256 variable-output-length ShakeHash. Its generic security
/// strength is 256 bits against all attacks if at least 64 bytes of its output are used.
pub fn new_shake256() -> SHAKE256 {
    SHAKE256 {
        state: State::Absorbing(Shake256::default()),
    }
}

/// shake_sum256 writes an arbitrary-length digest of data into hash.
pub fn shake_sum256(hash: &mut [u8], b: &[u8]) -> io::Result<usize> {
    let mut h = new_shake256();

    let _ = h.write(b).expect("unfallible");
    h.read(hash)
}
