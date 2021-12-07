use minecraft_protocol::cursor::prelude::*;
use minecraft_protocol::vec::native::WriteMCNativeTypes;
use minecraft_protocol::vec::string::WriteMCString;
use minecraft_protocol::vec::var_int::WriteMCVarInt;
use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::*;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    println!("connecting");
    let mut stream = TcpStream::connect("localhost:25565").await?;
    println!("connected");
    let (mut i, mut o) = stream.split();
    let mut buf = Vec::with_capacity(2097050);
    let mut conn_buf = Vec::with_capacity(2097050);
    conn_buf
        .write_var_int(757)
        .write_str("0.0.0.0")
        .write_u16be(25565)
        .write_var_int(1);
    conn_buf.prefix_var_int(conn_buf.len() as i32);
    println!(
        "writing handshake (state \"status\") packet\n{:#x?}",
        &conn_buf[..]
    );
    o.write_buf(&mut Cursor::new(&mut conn_buf[..])).await?;
    println!("writing status request packet");
    println!("{:#x?}", &[1, 0]);
    o.write_buf(&mut Cursor::new(&[1, 0])).await?;
    println!("writing ping packet");
    conn_buf.clear();
    conn_buf.write_u8be(1).write_i64be(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64,
    );
    conn_buf.prefix_var_int(conn_buf.len() as i32);
    println!("{:#x?}", &conn_buf[..]);
    o.write_buf(&mut Cursor::new(&mut conn_buf)).await?;
    loop {
        println!("waiting for read");
        let x = i.read(&mut buf[..]).await?;
        if x == 0 {
            println!("read packet size was 0, ending program.");
            break;
        }
        println!("read packet: {:#x?}", &buf[0..x]);
    }
    Ok(())
}

fn prefix_with_var_int(var_int: i32, buf: &mut [u8]) -> Vec<u8> {
    let mut var_int_buf = [0u8; 5];
    let mut var_int_cursor = Cursor::new(&mut var_int_buf);
    var_int_cursor.write_var_int(var_int).unwrap();
    let var_int_len = var_int_cursor.position() as usize;
    let buf_len = buf.len();
    let mut new_buf = vec![0; var_int_len + buf_len];
    new_buf[0..var_int_len].copy_from_slice(&var_int_buf[0..var_int_len]);
    new_buf[var_int_len..var_int_len + buf_len].copy_from_slice(buf);
    new_buf
}
