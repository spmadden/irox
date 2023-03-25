use std::net::UdpSocket;
use bytecodec::{DecodeExt, EncodeExt, Error};
use stun_codec::{Message, MessageClass, MessageDecoder, MessageEncoder, TransactionId};
use stun_codec::rfc5389::{attributes::Software, methods::BINDING, Attribute};


fn main() {

// Creates a message
    let mut message = Message::new(MessageClass::Request, BINDING, TransactionId::new([3; 12]));
    message.add_attribute(Attribute::Software(Software::new("foo".to_owned()).expect("error")));

// Encodes the message
    let mut encoder = MessageEncoder::new();
    let bytes = encoder.encode_into_bytes(message.clone()).expect("error");

    println!("{:?}", bytes);

    let socket = UdpSocket::bind("0.0.0.0:7777").expect("error");
    println!("Sending packet to google.");
    socket.send_to(bytes.as_slice(), "stun.l.google.com:19302").expect("error");

    println!("Sent, waiting for response");
    let mut recvd : [u8;1500] = [0;1500];
    let (size, rsock) = socket.recv_from(&mut recvd).expect("error");
    println!("Received {}, {} from google", size, rsock);
    let vec : Vec<u8> = Vec::from(&recvd[0..size]);

    let mut decoder = MessageDecoder::<Attribute>::new();
    let decoded = decoder.decode_from_bytes(vec.as_slice()).expect("error").map_err(Error::from).expect("error");

    println!("{:?}", decoded);
}
