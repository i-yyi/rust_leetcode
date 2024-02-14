use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut e: HashMap<String, Vec<String>> = HashMap::new();
        for edge in tickets.into_iter() {
            let (to, from) = (edge.remove(1), edge.remove(0));
            e.entry(edge[0]).or_insert(vec![]).push(edge[1]);
        }
        for (from, to) in &mut e {
            let mut v = to;
            v.sort();
        }
        vec![]
    }
}