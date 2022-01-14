use std::{
    io::{self, BufRead as _},
    rc::Rc,
};

#[derive(Debug)]
struct Cave {
    small: bool,
    label: Box<str>,
    goes_to: Vec<usize>,
}

#[derive(Default, Debug)]
struct Graph {
    caves: Vec<Cave>,
}

impl Graph {
    fn find_node(&self, label: &str) -> Option<usize> {
        self.caves.iter().position(|c| c.label.as_ref() == label)
    }

    fn alloc_node(&mut self, label: &str) -> usize {
        match self.find_node(label) {
            Some(pos) => pos,
            None => {
                self.caves.push(Cave {
                    small: label.chars().all(|c| c.is_lowercase()),
                    label: label.into(),
                    goes_to: Default::default(),
                });
                self.caves.len() - 1
            }
        }
    }

    fn add_pass(&mut self, pass: &str) {
        let (from, to) = pass.split_once('-').unwrap();
        let to_idx = self.alloc_node(to);
        let from_idx = self.alloc_node(from);
        // We assume there are no multiple arcs
        self.caves[from_idx].goes_to.push(to_idx);
        self.caves[to_idx].goes_to.push(from_idx);
    }
}

#[derive(Debug)]
struct Path {
    cave: usize,
    prev: Option<Rc<Path>>,
}

impl Path {
    fn visited(&self, cave: usize) -> bool {
        self.cave == cave || self.prev.as_ref().map(|p| p.visited(cave)).unwrap_or(false)
    }
}

fn graph_search(graph: &Graph) -> usize {
    let mut qju = Vec::<Rc<Path>>::new();
    let mut cnt = 0;
    let end = graph.find_node("end").unwrap();

    qju.push(Rc::new(Path {
        cave: graph.find_node("start").unwrap(),
        prev: None,
    }));
    while let Some(path) = qju.pop() {
        if path.cave == end {
            cnt += 1;
        } else {
            // end is small, so we cannot pass multiple times; thus "else"
            qju.extend(
                graph.caves[path.cave]
                    .goes_to
                    .iter()
                    .cloned()
                    .filter(|&idx| !graph.caves[(idx)].small || !path.visited(idx))
                    .map(|idx| {
                        Rc::new(Path {
                            cave: idx,
                            prev: Some(path.clone()),
                        })
                    }),
            );
        }
    }
    cnt
}

fn main() {
    let mut graph: Graph = Default::default();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        graph.add_pass(&line);
    }
    println!("{}", graph_search(&graph));
}
