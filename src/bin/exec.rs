use std::io;

use swayrc::wrap;

pub fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len()<3 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "not enough arguments"));
    }
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    wrap::special_exec(args[1], &args[2..])
}
