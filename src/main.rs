use modding::client::Client;
use std::time::Instant;

fn main() {
    let client = Client::load("mods");
    let start = Instant::now();
    // println!("{:#?} | elapsed {:?}", client, start.elapsed());
    println!("{} | elapsed {:?}", client.get_translation("test/en", "ready_for_today"), start.elapsed());
    // println!("{} | elapsed {:?}", client.base.language_default().get("ready_for_today"), start.elapsed());
}