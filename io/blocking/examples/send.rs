extern crate magic_wormhole_io_blocking;
extern crate hex;
use magic_wormhole_io_blocking::Wormhole;

// Can ws do hostname lookup? Use ip addr, not localhost, for now
const MAILBOX_SERVER: &'static str = "ws://127.0.0.1:4000/v1";
const APPID: &'static str = "lothar.com/wormhole/text-or-file-xfer";

fn main() {
    let mut w = Wormhole::new(APPID, MAILBOX_SERVER);
    println!("connecting..");
    w.set_code("4-purple-sausages");
    println!("sending..");
    w.send_message(b"hello");
    println!("sent..");
    // if we close right away, we won't actually send anything. Wait for at
    // least the verifier to be printed, that ought to give our outbound
    // message a chance to be delivered.
    let verifier = w.get_verifier();
    println!("verifier: {}", hex::encode(verifier));
    println!("got verifier, closing..");
    w.close();
    println!("closed");
}
