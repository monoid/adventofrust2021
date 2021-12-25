fn main() {
    let mut map = lib::read_map().unwrap();

    for i in 1.. {
        let has_moves = lib::advance(&mut map);

        if !has_moves {
            println!("{}", i);
            break;
        }
        // lib::print_map(&map);
    }
}
