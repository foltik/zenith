use stud::{args::clap, error::Result, log};

/// Example UDP multicast chat app
#[derive(Debug, clap::Parser)]
struct Args {
    /// Chat username
    #[arg(short, long, default_value = "anonymous")]
    username: String,

    /// UDP port number
    #[arg(short, long, default_value_t = 53507)]
    port: u16,

    /// Listen IP address
    #[arg(short, long, default_value = "0.0.0.0")]
    listen_addr: Ipv4Addr,

    /// Multicast IP address
    #[arg(short, long, default_value = "239.255.42.98")]
    mcast_addr: Ipv4Addr,
}

use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UdpSocket,
    task,
};

/// Bind socket to multicast address with IP_MULTICAST_LOOP and SO_REUSEADDR Enabled
fn bind_multicast(addr: &SocketAddrV4, mcast_addr: &SocketAddrV4) -> Result<std::net::UdpSocket> {
    use socket2::{Domain, Protocol, Socket, Type};

    assert!(mcast_addr.ip().is_multicast(), "Must be multcast address");

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

    socket.set_reuse_address(true)?;
    socket.bind(&socket2::SockAddr::from(*addr))?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(mcast_addr.ip(), addr.ip())?;

    Ok(socket.try_into()?)
}

/// Receive bytes from UPD socket and write to stdout until EOF.
async fn receive(sock: Arc<UdpSocket>) -> Result<()> {
    let mut buffer = vec![0u8; 4096];
    let mut stdout = tokio::io::stdout();

    loop {
        let n = sock.recv(&mut buffer[..]).await?;
        if n == 0 {
            break;
        }
        stdout.write_all(&mut buffer[..n]).await?;
    }

    Ok(())
}

/// Transmit bytes from stdin until EOF, Ctrl+D on linux or Ctrl+Z on windows.
async fn transmit(sock: Arc<UdpSocket>, addr: SocketAddr, mut username: String) -> Result<()> {
    username.push_str(": ");
    let mut buffer = username.into_bytes();
    let l = buffer.len();
    buffer.resize(4096, 0);

    let mut stdin = tokio::io::stdin();
    loop {
        let n = stdin.read(&mut buffer[l..]).await?;
        if n == 0 {
            break;
        }
        sock.send_to(&mut buffer[..l + n], &addr).await?;
    }

    Ok(())
}

#[zenith::main(args = Args)]
async fn main(args: Args) -> Result<()> {
    log::debug!("{args:?}");

    let username = args.username;
    let addr = SocketAddrV4::new(args.listen_addr, args.port);
    let mcast_addr = SocketAddrV4::new(args.mcast_addr, args.port);

    log::info!("Listen address: {addr}");
    log::info!("Multicast address: {mcast_addr}");

    let std_socket = bind_multicast(&addr, &mcast_addr).expect("Failed to bind multicast socket");
    // bind_multicast(&addr, &mcast_addr).context("Failed to bind multicast socket")?;

    let socket = Arc::new(UdpSocket::try_from(std_socket).unwrap());
    let (udp_rx, udp_tx) = (Arc::clone(&socket), socket);

    tokio::select! {
        res = task::spawn(async move { receive(udp_rx).await }) => res?,
        res = task::spawn(async move { transmit(udp_tx, mcast_addr.into(), username).await }) => res?,
    }
}
