use rand::Rng;
use zxcvbn::zxcvbn;

use crate::opts::GeneratePasswordOpts;

pub fn process_gen_pass(opts: GeneratePasswordOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();
    if opts.uppercase {
        chars.extend_from_slice(b"ABCDEFGHJKLMNPQRSTUVWXYZ");
    }
    if opts.lowercase {
        chars.extend_from_slice(b"abcdefghijkmnpqrstuvwxyz");
    }
    if opts.number {
        chars.extend_from_slice(b"123456789");
    }
    if opts.symbol {
        chars.extend_from_slice(b"!@#$%^&*()_+-=[]{}|;':\",./<>?");
    }
    for _ in 0..opts.length {
        password.push(chars[rng.gen_range(0..chars.len())] as char);
    }
    println!("{}", password);
    let estimate = zxcvbn("123", &[]);
    println!("{}", estimate.score());
    Ok(())
}
