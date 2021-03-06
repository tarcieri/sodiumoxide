#![macro_escape]
macro_rules! hash_module (($hash_name:ident, $hashbytes:expr, $blockbytes:expr) => (

#[link(name = "sodium")]
extern {
    fn $hash_name(h: *mut u8,
                  m: *const u8,
                  mlen: c_ulonglong) -> c_int;
}

pub const HASHBYTES: uint = $hashbytes;
pub const BLOCKBYTES: uint = $blockbytes;

/**
 * Digest-structure
 */
pub struct Digest(pub [u8, ..HASHBYTES]);

newtype_clone!(Digest)

/**
 * `hash` hashes a message `m`. It returns a hash `h`.
 */
pub fn hash(m: &[u8]) -> Digest {
    unsafe {
        let mut h = [0, ..HASHBYTES];
        $hash_name(h.as_mut_ptr(), m.as_ptr(), m.len() as c_ulonglong);
        Digest(h)
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use randombytes::randombytes;
    use super::*;

    const BENCH_SIZES: [uint, ..14] = [0, 1, 2, 4, 8, 16, 32, 64,
                                       128, 256, 512, 1024, 2048, 4096];

    #[bench]
    fn bench_hash(b: &mut test::Bencher) {
        let ms: Vec<Vec<u8>> = BENCH_SIZES.iter().map(|s| {
            randombytes(*s)
        }).collect();
        b.iter(|| {
            for m in ms.iter() {
                hash(m.as_slice());
            }
        });
    }
}

))
