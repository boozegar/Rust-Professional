use std::collections::HashMap;
use std::str::FromStr;

struct TwoWayTable {
    base_map1: HashMap<char, u32>,
    base_map2: HashMap<u32, char>,
}
impl TwoWayTable {
    fn create() -> Self {
        let mut l: [(char, u32); 16] = [('a', 0); 16];
        for i in 0..10 {
            l[i] = (i.to_string().chars().next().unwrap(), i as u32);
        }
        l[10] = ('a', 10);
        l[11] = ('b', 11);
        l[12] = ('c', 12);
        l[13] = ('d', 13);
        l[14] = ('e', 14);
        l[15] = ('f', 15);

        let mut base_map1 = HashMap::from(l);
        let mut base_map2 = HashMap::new();
        for (k, v) in &base_map1 {
            base_map2.insert(*v, *k);
        }
        TwoWayTable {
            base_map1,
            base_map2,
        }
    }
    fn get_by_char(&self, c: &char) -> u32 {
        self.base_map1[c]
    }
    fn get_by_usize(&self, u: &u32) -> char {
        self.base_map2[u]
    }
}

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let args: Vec<&str> = num_str
        .split(|c| c == ',' || c == '(' || c == ')')
        .collect();
    let (nums_chars, from_base): (Vec<char>, _) =
        (args[0].chars().collect(), u32::from_str(args[1]).unwrap());
    let twt = TwoWayTable::create();
    // TODO: 这里写逻辑
    // ("10(2)", 10, "2"),("12(10)", 16, "c"),
    let mut mid_sum = 0u32;
    for i in 0..nums_chars.len() {
        let key = &nums_chars[nums_chars.len() - 1 - i];
        mid_sum += u32::pow(from_base, i as u32) * twt.get_by_char(key);
    }
    let mut res = String::new();
    loop {
        let mut remainder;
        (mid_sum, remainder) = (mid_sum / to_base, mid_sum % to_base);
        res.push_str(&*(twt.get_by_usize(&remainder).to_string()));
        if mid_sum == 0{
            break;
        }
        
    }
    // println!("res:{:?}", res);
    res.chars().rev().collect()
}
