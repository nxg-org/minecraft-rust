use std::time::{SystemTime, UNIX_EPOCH};

use bytes::BufMut;
use clap::Parser;
use minecraft_protocol::{prelude::*, ReadMCNativeTypes};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct App {
    address: String,
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let address = &App::parse().address;

    let mut handshake = vec![];
    handshake.put_var_int(0);
    handshake.put_var_int(-1);
    handshake.put_str(address);
    handshake.put_u16(25565);
    handshake.put_var_int(1);
    prefix_packet_len_var_int(&mut handshake);

    let mut status = vec![];
    status.put_u8(0);
    prefix_packet_len_var_int(&mut status);

    let mut ping = vec![];
    ping.put_u8(1);
    ping.put_i64(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
    );
    prefix_packet_len_var_int(&mut ping);

    let mut socket = TcpStream::connect(address).await?;

    socket.write_all(&handshake).await?;
    socket.write_all(&status).await?;
    socket.write_all(&ping).await?;

    println!("written packets");

    let mut bmut = bytes::BytesMut::with_capacity(2097050);
    loop {
        if let Ok(x) = socket.read_buf(&mut bmut).await {
            if x == 0 {
                break println!("Server closed connection.");
            }
            println!("read packet: {:?}", bmut.as_ref());
            let packet_len = bmut.get_var_int();
            println!("packet length: {}", &packet_len);
            let packet_id = bmut.get_var_int();
            println!("packet_id: {}", packet_id);
            match packet_id {
                0 => {
                    println!("Response from Server:\n{}", &bmut.get_str());
                }
                _ => println!("received packet other than server list ping response"),
            }

            bmut.clear();
        }
    }
    Ok(())
}

fn prefix_packet_len_var_int(packet_buf: &mut Vec<u8>) {
    let len = packet_buf.len();
    packet_buf.put_var_int(len as i32);
    let len_var_int = packet_buf[len..].to_vec();
    let len_var_int_len = len_var_int.len();
    packet_buf.copy_within(..len, len_var_int_len);
    packet_buf[..len_var_int_len].copy_from_slice(&len_var_int);
}
