use std::{
    io::{Read, stdin},
    ops::{Index, IndexMut},
};

const MAX_VALUE: u32 = 300000;

fn main() {
    // Get guests from user
    let (guests_count, guests) = get_guests_from_user();

    let mut nodes = Nodes::new(guests, guests_count);

    for guest_index in (0..guests_count).map(|i| i as u32) {
        let height_guest_index = nodes[guest_index].height + guests_count - 1;
        let weight_guest_index = nodes[guest_index].weight + guests_count - 1 + MAX_VALUE;
        nodes.union(guest_index, height_guest_index);
        nodes.union(guest_index, weight_guest_index);
    }

    // Count divisions
    let mut parents: u32 = 0;
    for guest_index in (0..guests_count).map(|i| i as u32) {
        if guest_index == nodes.parents[guest_index as usize] {
            parents += 1;
        }
    }

    println!("{}", parents - 1);
}

#[derive(Clone, Copy, Debug)]
struct Guest {
    height: u32,
    weight: u32,
}

impl Guest {
    fn new(height: u32, weight: u32) -> Self {
        Self {
            height: height,
            weight: weight,
        }
    }
}

fn get_guests_from_user() -> (u32, Vec<Guest>) {
    // Read all the input as bytes
    let mut input = Vec::new();
    stdin().read_to_end(&mut input).unwrap();
    let bytes: &[u8] = &input;
    let mut pos: usize = 0;

    // Get guests count
    let guests_count: u32 = next_value(bytes, &mut pos);

    // Get guests
    let mut guests: Vec<Guest> = Vec::with_capacity(guests_count as usize);
    for _ in 0..guests_count {
        let height: u32 = next_value(bytes, &mut pos);
        let weight: u32 = next_value(bytes, &mut pos);
        let new_guest = Guest::new(height, weight);
        guests.push(new_guest);
    }

    (guests_count, guests)
}

fn next_value(bytes: &[u8], pos: &mut usize) -> u32 {
    while !bytes[*pos].is_ascii_digit() {
        *pos += 1;
    }

    let mut value: u32 = 0;
    while bytes[*pos].is_ascii_digit() {
        value = value * 10 + (bytes[*pos] - b'0') as u32;
        *pos += 1;
    }

    value
}

struct Nodes {
    guests: Vec<Guest>,
    parents: Vec<u32>,
}

impl Nodes {
    fn new(guests: Vec<Guest>, guests_count: u32) -> Self {
        let parents: Vec<u32> = (0..MAX_VALUE * 2 + guests_count)
            .map(|i| i as u32)
            .collect();
        Self { guests, parents }
    }

    fn root_parent_index(self: &mut Self, mut guest_index: u32) -> u32 {
        let mut root_parent_index = guest_index;
        while self.parents[root_parent_index as usize] != root_parent_index {
            root_parent_index = self.parents[root_parent_index as usize];
        }

        // Flatten the tree
        while self.parents[guest_index as usize] != root_parent_index {
            let parent_index_temp = self.parents[guest_index as usize];
            self.parents[guest_index as usize] = root_parent_index;
            guest_index = parent_index_temp;
        }

        root_parent_index
    }

    fn union(self: &mut Self, first_guest_index: u32, second_guest_index: u32) {
        let root_parent_first = self.root_parent_index(first_guest_index);
        let root_parent_second = self.root_parent_index(second_guest_index);
        if root_parent_first != root_parent_second {
            self.parents[root_parent_second as usize] = root_parent_first as u32;
        }
    }
}

impl Index<usize> for Nodes {
    type Output = Guest;

    fn index(&self, index: usize) -> &Self::Output {
        &self.guests[index]
    }
}

impl IndexMut<usize> for Nodes {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.guests[index]
    }
}

impl Index<u32> for Nodes {
    type Output = Guest;

    fn index(&self, index: u32) -> &Self::Output {
        &self.guests[index as usize]
    }
}

impl IndexMut<u32> for Nodes {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.guests[index as usize]
    }
}
