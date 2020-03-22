use io::Write;
use std::io;

unsafe fn write_to_shm(buf: &[u8], cstr: &std::ffi::CString, nbytes: usize) -> bool {
    use libc::{
        ftruncate, mmap, munmap, shm_open, shm_unlink, MAP_FAILED, MAP_SHARED, O_CREAT, O_RDWR,
        PROT_READ, PROT_WRITE,
    };
    use std::ptr::{copy_nonoverlapping, null_mut};

    shm_unlink(cstr.as_ptr());
    let fd = shm_open(cstr.as_ptr(), O_RDWR | O_CREAT, 0777);
    if fd == -1 {
        eprintln!(">:(1 {}", *libc::__errno_location());
        return false;
    }

    let res = ftruncate(fd, nbytes as i64);
    if res == -1 {
        eprintln!(">:(2");
        shm_unlink(cstr.as_ptr());
        return false;
    }

    let fb = mmap(
        null_mut(),
        nbytes,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fd,
        0,
    );
    if fb == MAP_FAILED {
        eprintln!(">:(3");
        shm_unlink(cstr.as_ptr());
        return false;
    }

    copy_nonoverlapping(buf as *const [u8] as *const u8, fb as *mut u8, nbytes);

    let unmap_res = munmap(fb, nbytes);
    if unmap_res == -1 {
        eprintln!(">:(4");
        shm_unlink(cstr.as_ptr());
        return false;
    }

    return true;
}

pub fn display_shm(buf: &[u8], size: (usize, usize), bits_per_pixel: usize) -> io::Result<()> {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let chrs = "abcdefghijklmnopqrstuwvxyz0123456789";
    let salt: String = (0..48)
        .map(|_| rng.gen_range(0usize, chrs.len()))
        .map(|i| chrs.as_bytes()[i] as char)
        .collect();
    let name = format!("/kittyfb{}", salt);

    let cstr = std::ffi::CString::new(name.as_bytes()).unwrap();
    let nbytes = size.0 * size.1 * bits_per_pixel / 8;
    let shm_ok = unsafe { write_to_shm(buf, &cstr, nbytes) };

    if shm_ok {
        let cmd = format!(
            "\x1b_Gf={},s={},v={},a=T,t=s;",
            bits_per_pixel, size.0, size.1
        );
        let payload = base64::encode(cstr.into_bytes());

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(&cmd.as_bytes())?;
        handle.write_all(&payload.as_bytes())?;
        handle.write_all(b"\x1b\\")?;
        handle.flush()?;

        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "couldn't create a shared memory object",
        ))
    }
}
