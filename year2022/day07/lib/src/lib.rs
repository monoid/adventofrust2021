use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, not_line_ending},
    combinator::{all_consuming, map, value},
    multi::separated_list0,
    sequence::{pair, preceded, separated_pair, terminated},
};

#[derive(Debug)]
pub struct Shell {
    cwd: Vec<Rc<RefCell<Dir>>>,
}

impl Shell {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(Dir::default()));
        Self { cwd: vec![root] }
    }

    pub fn execute(&mut self, command: Command) {
        match command {
            Command::CdParent => {
                // Never pop root dir
                if self.cwd.len() > 1 {
                    self.cwd.pop();
                }
            }
            Command::CdDir(path) => {
                if path == b"/" {
                    self.cwd.truncate(1);
                } else {
                    let new_path = {
                        let top_dir = self.cwd.last().unwrap();
                        let dir_ref = top_dir.borrow_mut();
                        dir_ref.getdir(&path).unwrap()
                    };
                    self.cwd.push(new_path);
                }
            }
            Command::Ls(dir) => {
                let top_dir = self.cwd.last().unwrap();
                let mut dir_ref = top_dir.borrow_mut();
                *dir_ref = dir;
            }
        }
    }

    pub fn into_root(self) -> Rc<RefCell<Dir>> {
        self.cwd.into_iter().next().unwrap()
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Entry {
    File(u32),
    Dir(Rc<RefCell<Dir>>),
}

impl Entry {
    fn size(&self) -> u32 {
        match self {
            Entry::File(size) => *size,
            Entry::Dir(ref dir_cell) => {
                let dir = dir_cell.borrow();
                dir.size()
            }
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Dir {
    children: HashMap<Vec<u8>, Entry>,
}

impl Dir {
    pub fn size(&self) -> u32 {
        self.children.values().map(Entry::size).sum()
    }

    pub fn walk_size(&self, callback: &mut impl FnMut(u32)) -> u32 {
        let size: u32 = self
            .children
            .values()
            .map(|ent| match ent {
                Entry::File(size) => *size,
                Entry::Dir(dir_cell) => {
                    let dir = dir_cell.borrow();
                    dir.walk_size(callback)
                }
            })
            .sum();
        callback(size);
        size
    }

    fn getdir(&self, name: &[u8]) -> Option<Rc<RefCell<Dir>>> {
        self.children.get(name).and_then(|entry| match entry {
            Entry::File(_) => None,
            Entry::Dir(d) => Some(d.clone()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    CdParent,
    CdDir(Vec<u8>),
    Ls(Dir),
}

pub fn parse_commands(inp: &[u8]) -> nom::IResult<&[u8], Vec<Command>> {
    all_consuming(terminated(separated_list0(newline, parse_command), newline))(inp)
}

fn parse_command(inp: &[u8]) -> nom::IResult<&[u8], Command> {
    preceded(
        tag("$ "),
        alt((
            preceded(
                tag("cd "),
                alt((
                    value(Command::CdParent, tag("..")),
                    map(not_line_ending, |s: &[u8]| Command::CdDir(s.to_owned())),
                )),
            ),
            map(
                preceded(
                    pair(tag("ls"), newline),
                    separated_list0(newline, parse_ls_output),
                ),
                |data| {
                    Command::Ls(Dir {
                        children: data.into_iter().collect(),
                    })
                },
            ),
        )),
    )(inp)
}

fn parse_ls_output(inp: &[u8]) -> nom::IResult<&[u8], (Vec<u8>, Entry)> {
    use nom::character::complete::u32;
    alt((
        map(preceded(tag("dir "), not_line_ending), |name: &[u8]| {
            (name.to_owned(), Entry::Dir(Default::default()))
        }),
        map(
            separated_pair(u32, tag(" "), not_line_ending),
            |(size, name): (u32, &[u8])| (name.to_owned(), Entry::File(size)),
        ),
    ))(inp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_cdparent() {
        assert_eq!(
            parse_command(&b"$ cd .."[..]),
            Ok((&b""[..], Command::CdParent))
        );
    }

    #[test]
    fn test_parse_command_cddir() {
        assert_eq!(
            parse_command(&b"$ cd somedir"[..]),
            Ok((&b""[..], Command::CdDir((&b"somedir"[..]).to_owned())))
        );
    }

    #[test]
    fn test_parse_command_cdroot() {
        assert_eq!(
            parse_command(&b"$ cd /"[..]),
            Ok((&b""[..], Command::CdDir((&b"/"[..]).to_owned())))
        );
    }

    #[test]
    fn test_parse_command_ls() {
        assert_eq!(
            parse_command(&b"$ ls\ndir name\n42 file"[..]),
            Ok((
                &b""[..],
                Command::Ls(Dir {
                    children: maplit::hashmap! {
                        (&b"name"[..]).to_owned() => Entry::Dir(Rc::new(RefCell::new(Dir {
                            children: Default::default()
                        }))),
                        (&b"file"[..]).to_owned() => Entry::File(42),
                    }
                })
            ))
        );
    }
}
