use std::io;
use io::Write;

pub fn reset_cursor() -> io::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(b"\x1b[;H")?;
    handle.flush()?;

    Ok(())
}

pub fn clear() -> io::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(b"\x1b_Ga=d\x1b\\")?;
    handle.flush()?;

    reset_cursor()?;

    Ok(())
}
