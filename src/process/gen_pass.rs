use rand::prelude::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIGKLMN";
const LOWER: &[u8] = b"abcdefghjkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"!@#$%&_";

pub fn gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("upper won't be empty"))
    };
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("upper won't be empty"))
    };
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("upper won't be empty"))
    };
    if symbol {
        chars.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("upper won't be empty"))
    };
    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).unwrap();
        password.push(*c);
    }
    password.shuffle(&mut rng);
    println!("{}", String::from_utf8(password)?);
    Ok(())
}
