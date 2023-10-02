use join_string::Join;
use std::fs::File;

const USAGE: &str = "<file> <separator> [string]...";

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);

    let filename = args.next().expect(USAGE);
    let sep = args.next().expect(USAGE);

    let file = File::create(filename)?;

    args.join(sep).write_io(file)?;

    Ok(())
}
