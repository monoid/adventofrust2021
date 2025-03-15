use std::io::BufRead;

use lib::{range_checksum, Count, Id};

fn main() {
    let mut disk_map = DiskMap::read_map(std::io::stdin().lock());
    disk_map.pack();
    let checksum = disk_map.checksum();
    println!("{checksum}");
}

struct DiskMap {
    items: Vec<Item>,
}

impl DiskMap {
    pub fn read_map<R: BufRead>(mut inp: R) -> Self {
        let mut buf = String::new();
        inp.read_line(&mut buf).unwrap();
        let buf = buf.trim();

        let items = buf
            .trim()
            .chars()
            .enumerate()
            .map(|(id, c)| {
                let size: Count = c.to_digit(10).unwrap().try_into().unwrap();
                if id % 2 == 0 {
                    let id = id as Id / 2;
                    Item::file(id, size)
                } else {
                    Item::free(size)
                }
            })
            .collect();

        Self { items }
    }

    fn pack(&mut self) {
        for idx in (0..self.items.len()).rev() {
            if let &Item::File(file) = &self.items[idx] {
                for possible_fit in 0..idx {
                    if let Item::Free(free) = &mut self.items[possible_fit] {
                        if free.try_to_add(file).is_none() {
                            // TODO am I right that joining free cells is not needed because
                            // next iteration will not try this space and the adjacent one anyway?
                            self.items[idx] = Item::free(file.size);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn checksum(&self) -> u64 {
        let mut start = 0;
        let mut sum = 0;

        for item in &self.items {
            let (item_sum, new_start) = item.checksum(start);
            sum += item_sum;
            start = new_start;
        }

        sum
    }
}

#[derive(Copy, Clone, Debug)]
struct File {
    id: Id,
    size: Count,
}

impl File {
    fn checksum(&self, start: usize) -> (u64, usize) {
        (range_checksum(start, self.size, self.id), start + self.size)
    }
}

#[derive(Debug)]
struct Free {
    size: Count,
    added: Vec<File>,
}

impl Free {
    fn try_to_add(&mut self, file: File) -> Option<File> {
        if file.size <= self.size {
            self.size -= file.size;
            self.added.push(file);
            None
        } else {
            Some(file)
        }
    }

    fn nested_checksum(&self, mut start: usize) -> (u64, usize) {
        let mut sum = 0;
        for file in &self.added {
            let (file_sum, next_start) = file.checksum(start);
            sum += file_sum;
            start = next_start;
        }
        start += self.size;
        (sum, start)
    }
}

enum Item {
    File(File),
    Free(Free),
}

impl Item {
    fn file(id: Id, size: Count) -> Self {
        Self::File(File { id, size })
    }

    fn free(size: Count) -> Self {
        Self::Free(Free {
            size,
            added: vec![],
        })
    }

    fn checksum(&self, start: usize) -> (u64, usize) {
        match self {
            Item::File(file) => file.checksum(start),
            Item::Free(free) => free.nested_checksum(start),
        }
    }
}
