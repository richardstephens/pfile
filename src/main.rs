use std::{fmt, io};
use std::io::{IsTerminal, Write};
use std::process::exit;
use clap::Parser;
use sha2::{Digest, Sha256};

#[derive(clap::Parser)]
#[command(version, arg_required_else_help = true)]
struct Args {
    #[arg(short, long)]
    pub len: u16,
    #[arg(short, long)]
    pub seed: u64,
    #[arg(short, long, default_value_t = false)]
    pub quiet: bool,
}

struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
    where
        T: ?Sized + AsRef<[u8]> + 'a,
    {
        HexSlice(data.as_ref())
    }
}

impl fmt::Display for HexSlice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

pub fn bytes_to_hex_str(bytes: &[u8]) -> String {
    format!("{}", HexSlice::new(bytes))
}

fn main() {

    let args = Args::parse();
    let mut stdout = io::stdout();

    if !args.quiet && stdout.is_terminal() {
        eprintln!("stdout is a terminal, exiting");
        exit(1);
    }

    let mut r = fastrand::Rng::with_seed(args.seed);

    let mut hasher = Sha256::new();

    for _ii in 0..args.len {
        let mut d = vec![0_u8; 1048576];
        r.fill(&mut d);
        hasher.update(&d);
        if !args.quiet {
            stdout.write_all(&d).unwrap();
        }
    }
    let hashres = hasher.finalize();

    eprintln!("generated data sha256sum: {}", bytes_to_hex_str(hashres.as_slice()));
}
