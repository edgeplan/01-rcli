use clap::Parser;
use rcli::{
    gen_pass, process_csv, process_decode, process_encode, process_generate, process_text_sign,
    process_text_verify, Base64SubCommand, Command, Opts, TextSignFormat, TextSubCommand,
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opt = Opts::parse();
    match opt.command {
        Command::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?;
        }
        Command::GenPass(opts) => {
            let password = gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
            println!("{}", password);

            let estimate = zxcvbn(&password, &[]);
            eprintln!("password strong: {}", estimate.score());
        }
        Command::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;
                println!("{}", decoded);
            }
        },
        Command::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                process_text_sign(&opts.input, &opts.key, opts.format)?;
            }
            TextSubCommand::Verify(opts) => {
                let verified =
                    process_text_verify(&opts.input, &opts.key, &opts.sign, opts.format)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_generate(&opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        std::fs::write(name, &key[0])?;
                    }
                    TextSignFormat::ED25519 => {
                        let name = &opts.output;
                        std::fs::write(name.join("ed25519.sk"), &key[0])?;
                        std::fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    }

    Ok(())
}
