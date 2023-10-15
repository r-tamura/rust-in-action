use hickory_client::{
    client::{Client, SyncClient},
    rr::{DNSClass, Name, RData, RecordType},
    udp::UdpClientConnection,
};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "8.8.8.8:53".parse()?;
    let conn = UdpClientConnection::new(address)?;
    let client = SyncClient::new(conn);

    let name = Name::from_str("www.example.com.")?;

    let response = client.query(&name, DNSClass::IN, RecordType::A)?;

    let answers = response.answers();

    if let Some(RData::A(ref ip)) = answers[0].data() {
        println!("{}", ip);
    } else {
        println!("No A record found");
    }

    Ok(())
}
