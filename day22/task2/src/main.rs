fn main() {
    let commands = lib::prase_input();

    let mut scene: Vec<lib::cube::Cuboid> = vec![];
    for command in commands {
        scene = scene
            .into_iter()
            .flat_map(|c| c.subtract(&command.cuboid))
            .collect();
        if command.state {
            scene.push(command.cuboid.clone());
        }
    }
    eprintln!("{} cubes", scene.len());

    println!("{}", scene.iter().map(|c| c.volume()).sum::<usize>());
}
