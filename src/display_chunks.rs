use io::Write;
use std::io;

pub fn display_chunks(buf: &[u8], size: (usize, usize), bits_per_pixel: usize) -> io::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    for k in 0..(buf.len() / 4096 + 1) {
        let from = k * 4096;
        let to = ((k + 1) * 4096).min(buf.len());
        let m = to < buf.len();

        let cmd = if k == 0 {
            format!(
                "\x1b_Gf={},s={},v={},a=T,m={};",
                bits_per_pixel,
                size.0,
                size.1,
                if m { 1 } else { 0 }
            )
        } else if m {
            "\x1b_Gm=1;".into()
        } else {
            "\x1b_Gm=0;".into()
        };

        handle.write_all(&cmd.as_bytes())?;
        handle.write_all(&base64::encode(&buf[from..to]).as_bytes())?;
        handle.write_all(b"\x1b\\")?;
        handle.flush()?;
    }

    Ok(())
}
