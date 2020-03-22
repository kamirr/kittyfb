use io::Write;
use std::io;

pub fn reset_cursor() -> io::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(b"\x1b[;H")?;
    handle.flush()?;

    Ok(())
}
