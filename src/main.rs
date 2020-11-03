use modding::client::Client;
use std::time::Instant;

fn main() {
    let client = Client::load("mods");
    println!("{:#?} | elapsed {:?}", client, Instant::now().elapsed());
    println!("{} | elapsed {:?}", client.base.language_default().get("ready"), Instant::now().elapsed());
}