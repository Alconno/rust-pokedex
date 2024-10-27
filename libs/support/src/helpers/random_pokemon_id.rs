use rand::prelude::*;
use std::{
    collections::HashMap, sync::{Arc, Mutex}
};

pub async fn pick_from_blacklist(mut total_range: i32, blacklist: Vec<i32>) -> i32 {
    let mut map = HashMap::<i32, i32>::new();
    let rng = Arc::new(Mutex::new(StdRng::from_entropy()));

    for &blacklist_num in &blacklist {
        map.insert(blacklist_num, -1);
    }

    let pick_range = total_range - map.len() as i32;

    for blacklist_num in blacklist {
        if blacklist_num < pick_range {
            while map.contains_key(&(total_range - 1)) {
                total_range -= 1;
            }
            map.insert(blacklist_num, total_range - 1);
            total_range -= 1;
        }
    }

    let mut rng = rng.lock().unwrap();
    let generated_number = rng.gen_range(1..pick_range);

    if map.contains_key(&generated_number) {
        *map.get(&generated_number).unwrap()
    } else {
        generated_number
    }
}
