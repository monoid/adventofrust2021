use std::collections::VecDeque;
use std::io::BufRead;

use lib::{range_checksum, Count, Id};

fn main() {
    let disk_map = DiskMap1::read_map(std::io::stdin().lock());
    let cs = disk_map.checksum();
    println!("{cs}");
}

pub struct DiskMap1 {
    files: VecDeque<(Id, Count)>,
    free_regions_rev: Vec<Count>,
    pos: usize,
    checksum: u64,
}

impl DiskMap1 {
    pub fn read_map<R: BufRead>(mut inp: R) -> Self {
        let mut buf = String::new();
        inp.read_line(&mut buf).unwrap();
        let buf = buf.trim();

        let mut items = buf
            .trim()
            .chars()
            .enumerate()
            .map(|(id, c)| {
                let count: Count = c.to_digit(10).unwrap().try_into().unwrap();
                let id = id >> 1; // skip empty ranges at odd positions
                (id as Id, count)
            })
            .peekable();

        let mut files = VecDeque::with_capacity(buf.len() / 2 + 1);
        let mut free_regions = Vec::with_capacity(buf.len() / 2);

        files.push_back(items.next().unwrap());

        while items.peek().is_some() {
            let free = items.next().unwrap().1;
            let file = items.next().unwrap();

            free_regions.push(free);
            files.push_back(file);
        }

        free_regions.reverse();

        Self {
            files,
            free_regions_rev: free_regions,
            pos: 0,
            checksum: 0,
        }
    }

    pub fn checksum(mut self) -> u64 {
        while let Some(f) = self.files.pop_front() {
            self.checksum += range_checksum(self.pos, f.1, f.0);
            self.pos += f.1;

            // now there's a free space in front of us
            if let Some(free_size) = self.free_regions_rev.last_mut() {
                while *free_size > 0 {
                    if let Some(file) = self.files.back_mut() {
                        let len = std::cmp::min(*free_size, file.1);
                        self.checksum += range_checksum(self.pos, len, file.0);
                        self.pos += len;
                        file.1 -= len;
                        *free_size -= len;

                        if file.1 == 0 {
                            self.files.pop_back();
                        }
                        if *free_size == 0 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                self.free_regions_rev.pop();
            }
        }

        self.checksum
    }
}
