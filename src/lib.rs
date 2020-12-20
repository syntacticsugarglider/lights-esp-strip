use futures::{
    future::{ready, Either},
    stream::{once, unfold},
    Stream,
};
use smol::{io::AsyncWriteExt, Async};
use std::net::{TcpListener, TcpStream};
use std::{io::Error, net::IpAddr};

pub struct Light {
    socket: Async<TcpStream>,
    red: u8,
    green: u8,
    blue: u8,
}

impl Light {
    pub async fn turn_on(&mut self) -> Result<(), Error> {
        self.socket
            .write_all(format!("{},{},{}\n", self.red, self.green, self.blue).as_bytes())
            .await
    }
    pub async fn turn_off(&mut self) -> Result<(), Error> {
        self.socket.write_all("0,0,0".as_bytes()).await
    }
    pub async fn set_color(&mut self, color: (u8, u8, u8)) -> Result<(), Error> {
        self.red = color.0;
        self.green = color.1;
        self.blue = color.2;
        self.turn_on().await
    }
    pub fn addr(&self) -> Result<IpAddr, Error> {
        self.socket.get_ref().peer_addr().map(|addr| addr.ip())
    }
}

pub fn listen(port: u16) -> impl Stream<Item = Result<Light, Error>> {
    let socket = match TcpListener::bind(("0.0.0.0", port)) {
        Ok(socket) => socket,
        Err(e) => return Either::Left(once(ready(Err(e)))),
    };
    let socket = match Async::new(socket) {
        Ok(socket) => socket,
        Err(e) => return Either::Left(once(ready(Err(e)))),
    };
    Either::Right(unfold(socket, move |socket| async move {
        socket
            .accept()
            .await
            .map(|(stream, _)| {
                Some(Light {
                    red: 255,
                    green: 255,
                    blue: 255,
                    socket: stream,
                })
            })
            .transpose()
            .map(|item| (item, socket))
    }))
}