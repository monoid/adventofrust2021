use std::io::{self, Read};

#[derive(Debug)]
struct Node {
    version: u32,
    val: Val,
}

#[derive(Debug)]
enum Val {
    Literal(u32),
    Op(u32, Vec<Node>),
}

impl Node {
    fn version_sum(&self) -> u32 {
        self.version + self.val.version_sum()
    }
}

impl Val {
    fn version_sum(&self) -> u32 {
        match self {
            Val::Literal(_) => 0,
            Val::Op(_, ref items) => items.iter().map(Node::version_sum).sum::<u32>(),
        }
    }
}

fn iter_bits(inp: &'_ str) -> impl Iterator<Item = u8> + '_ {
    inp.chars().flat_map(|c| {
        let bits = match c {
            '0'..='9' => c as u8 - b'0',
            'A'..='F' => c as u8 - b'A' + 10,
            _ => panic!("Wrong digit {:?}", c),
        } as u8;
        (0..4).rev().map(move |pos| (bits >> pos) & 1u8)
    })
}

fn parse_num(it: &mut dyn Iterator<Item = u8>) -> u32 {
    it.fold(0u32, |p, b| (p << 1) | (b as u32))
}

fn parse_node(it: &mut dyn Iterator<Item = u8>) -> Node {
    let version = parse_num(&mut it.take(3));
    let type_ = parse_num(&mut it.take(3));

    let val = if type_ == 4 {
        parse_lit(it)
    } else {
        parse_op(type_, it)
    };

    Node { version, val }
}

fn parse_op(type_: u32, mut it: &mut dyn Iterator<Item = u8>) -> Val {
    let mut data = Vec::<Node>::new();

    if it.next().unwrap() == 0 {
        let len = parse_num(&mut it.take(15));
        let mut subit = it.take(len as usize).peekable();
        while subit.peek().is_some() {
            data.push(parse_node(&mut subit));
        }
    } else {
        let cnt = parse_num(&mut it.take(11));
        for _ in 0..cnt {
            data.push(parse_node(&mut it));
        }
    }

    Val::Op(type_, data)
}

fn parse_lit(it: &mut dyn Iterator<Item = u8>) -> Val {
    let mut res = 0;
    let mut pack;

    while {
        pack = parse_num(&mut it.take(5));
        pack & 0b10000 != 0
    } {
        res = (res << 4) | (pack & 0b01111);
    }
    res = (res << 4) | pack;

    Val::Literal(res)
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut it = iter_bits(input.trim());

    let node = parse_node(&mut it);
    println!("{}", node.version_sum());
}
