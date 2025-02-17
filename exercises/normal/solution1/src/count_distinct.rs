use std::collections::HashSet;
use std::hash::Hash;

pub fn new_count_distinct(input_str: &str) -> usize {
    // todo!()
    let strs:Vec<&str> = input_str.split(',').collect();
    let mut set = HashSet::new();
    for s in strs{
        set.insert(s);
    }
    set.len()
}


