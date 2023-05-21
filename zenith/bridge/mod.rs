use std::sync::OnceLock;


use crate::{Error, Result};

static BRIDGE: OnceLock<Bridge> = OnceLock::new();

impl Bridge {

}

#[derive(Debug, Error)]
pub enum InitError {
    #[error("foo")]
    Foo,
}

pub async fn init() -> Result<()> {
    // use smol::net::UdpSocket;
    // smol::spawn((async || -> smol::io::Result<()> {
    //     let sock = UdpSocket::bind("0.0.0.0:3507").await?;
    //     // sock.join_multicast_v4("224.0.0.16".parse().unwrap(), "0.0.0.0".parse().unwrap())?;

    //     unsafe {
    //         use std::os::fd::AsRawFd;
    //         let tru: libc::c_int = 1;
    //         let ret = libc::setsockopt(
    //             sock.as_raw_fd(),
    //             libc::SOL_SOCKET,
    //             libc::SO_REUSEADDR,
    //             &1 as *const _ as *const libc::c_void,
    //             std::mem::size_of::<libc::c_int>() as _,
    //         );
    //         if ret != 0 {
    //             return Err(std::io::Error::last_os_error())
    //         }
    //         let ret = libc::setsockopt(
    //             sock.as_raw_fd(),
    //             libc::SOL_SOCKET,
    //             libc::SO_REUSEPORT,
    //             &tru as *const _ as *const libc::c_void,
    //             std::mem::size_of::<libc::c_int>() as _,
    //         );
    //         if ret != 0 {
    //             return Err(std::io::Error::last_os_error())
    //         }
    //     }

    //     // sock.send_to("hello".as_bytes(), "224.0.0.16:3507").await?;

    //     loop {
    //         let mut buf = [0u8; 32];
    //         let (n, addr) = sock.recv_from(&mut buf).await?;

    //         log::info!("received {n} bytes from {addr}");

    //         smol::Timer::after(std::time::Duration::from_secs(1)).await;
    //     }
    // })()).await?;
    Ok(())
}
