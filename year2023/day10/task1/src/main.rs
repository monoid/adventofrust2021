fn main() {
    let map = lib::read_data();
    let start = lib::find_start_pos(&map).unwrap();
    let trace = lib::trace(start, &map);
    println!("{:?}", trace.len() / 2);
}
