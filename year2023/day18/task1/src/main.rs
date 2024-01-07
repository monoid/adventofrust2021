fn main() {
    let data = lib::read_data();
    let mut turtle = lib::Turtle::new();
    for instr in &data {
        turtle.move_(instr);
    }

    let mut cnt = 0;

    for (_, line) in turtle.into_sorted_track() {
        let line_cnt = lib::count_line(&line);
        cnt += line_cnt;
    }

    println!("{cnt}");
}
