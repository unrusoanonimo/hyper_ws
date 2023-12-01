use std::{collections::HashMap, hash::Hash};

pub fn count_map<T, I>(i: I) -> HashMap<T, usize>
where
    T: Eq + Hash,
    I: Iterator<Item = T>,
{
    let mut m = HashMap::new();
    i.into_iter().for_each(|k| {
        if let Some(v) = m.get_mut(&k) {
            *v += 1;
        } else {
            m.insert(k, 1);
        }
    });
    m
}
