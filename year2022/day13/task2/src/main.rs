fn main() {
    let data = lib::read();
    let mut flat_data: Vec<_> = data.into_iter().flat_map(|(a, b)| vec![a, b]).collect();
    let marker1 = lib::Node::List(vec![lib::Node::Number(2)]);
    let marker2 = lib::Node::List(vec![lib::Node::Number(6)]);
    flat_data.push(marker1.clone());
    flat_data.push(marker2.clone());
    flat_data.sort_unstable_by(lib::Node::compare);
    let idx1 = flat_data
        .binary_search_by(|n| lib::Node::compare(n, &marker1))
        .unwrap()
        + 1;
    let idx2 = flat_data
        .binary_search_by(|n| lib::Node::compare(n, &marker2))
        .unwrap()
        + 1;

    println!("{}", idx1 * idx2);
}
