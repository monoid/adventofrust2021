// This is not a satisfactory solution as it runs for 8 minutes
// and consumes gigabytes of memory.
fn main() {
    let data = lib::read_data2();
    let mut turtle = lib::Turtle::new();
    for instr in &data {
        turtle.move_(dbg!(instr));
    }
    eprintln!("turtle done...");

    let mut cnt = 0;

    for (_, line) in turtle.into_sorted_track() {
        let line_cnt = lib::count_line(&line);
        cnt += line_cnt;
    }

    println!("{cnt}");
}
