use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::cmp::Ordering;
//use std::fmt;
use std::usize;


#[derive(Debug, Clone, Copy)]
struct Pos {
    no1    : usize,
    no2    : usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.no1 == other.no1 && self.no2 == other.no2
    }
}

impl Eq for Pos {}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.no1.hash(state);
        self.no2.hash(state);
    }
}

#[derive(Eq, PartialEq)]
struct Node {
    no     : usize,
    cost   : usize,
    prev_no: Option<usize>,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.no.cmp(&other.no))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct WorldMap {
    nodes: HashMap<usize, Vec<Node>>,
    cache: RefCell<HashMap<Pos, usize>>,
}

impl WorldMap {
    fn new(map: &Vec<Vec<String>>) -> WorldMap {
        let mut nodes: HashMap<usize, Vec<Node>> = HashMap::new();
        let mut cache: HashMap<Pos, usize> = HashMap::new();

        for (_, l) in map.iter().enumerate() {
            let no1 = l[0].parse::<usize>().unwrap();
            let no2 = l[1].parse::<usize>().unwrap();
            let cost = l[2].parse::<usize>().unwrap();

            nodes.entry(no1).or_insert(Vec::new()).push(Node {no: no2, cost, prev_no: None});
            nodes.entry(no2).or_insert(Vec::new()).push(Node {no: no1, cost, prev_no: None});

            let pos = WorldMap::make_pos(no1, no2);
            cache.entry(pos).or_insert(cost);
        }

        WorldMap {
            nodes,
            cache: RefCell::new(cache),
        }
    }

    fn set_cache_pos(&self, pos: &Pos, cost: usize) {
        self.cache.borrow_mut().entry(*pos).or_insert(cost);
    }

    fn set_cache(&self, no1: usize, no2: usize, cost: usize) {
        let pos = WorldMap::make_pos(no1, no2);
        self.set_cache_pos(&pos, cost);
    }

    fn make_pos(no1: usize, no2: usize) -> Pos {
        Pos {no1: std::cmp::min(no1, no2), no2: std::cmp::max(no1, no2)}
    }

    fn cost(&self, start: usize, goal: usize) -> Option<usize> {
        let pos = WorldMap::make_pos(start, goal);
        if let Some(c) = self.cache.borrow().get(&pos) {
            //println!("cached! {:?}", *c);
            return Some(*c);
        }

        let mut heap = BinaryHeap::new();
        heap.push(Node {no: start, cost: 0, prev_no: None});

        while let Some(Node {no, cost, prev_no}) = heap.pop() {
            self.set_cache(start, no, cost);

            if no == goal {
                return Some(cost);
            }

            let nexts = self.nodes.get(&no).unwrap();

            for next in nexts.iter() {
                if let Some(prev_no) = prev_no {
                    if prev_no == next.no {
                        continue;
                    }
                }

                self.set_cache(no, next.no, next.cost);

                let nx = Node {no: next.no, cost: cost + next.cost, prev_no: Some(no)};
                heap.push(nx);
            }
        }

        None
    }

    fn costs(&self) -> usize {
        let mut costs: usize = 0;
        for i in 1..self.nodes.len()+1 {
            for j in i+1..self.nodes.len()+1 {
                if let Some(c) = self.cost(i, j) {
                    costs += c;
                }
            }
        }

        costs
    }

    fn avg(&self) -> f64 {
        let costs = self.costs();
        let len = self.nodes.len();
        let num = (len * (len-1)) / 2;
        (costs as f64) / (num as f64)
    }
}


fn main() {
    let map = read_map();
    let wmap = WorldMap::new(&map);
    let avg = wmap.avg();
    println!("{}", avg);
}

fn read_map() -> Vec<Vec<String>> {
    let h = read::<String>();
    let map = read_vec2::<String>(h.parse::<u32>().unwrap()-1);

    map
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_map(s: &str) -> Vec<Vec<String>> {
        let mut lines = s.lines();

        let _n: Vec<String> = lines.next().unwrap().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect();

        let map: Vec<Vec<String>> = lines.map(|l| l.split_whitespace()
                                              .map(|e| e.parse().ok().unwrap()).collect()).collect();

        map
    }
    
    #[test]
    fn test_1() {
        let s = "\
4
1 3 4
2 1 6
4 2 3";

        let map = make_map(s);
        let wmap = WorldMap::new(&map);

        assert_eq!(wmap.cost(1, 2), Some(6));
        assert_eq!(wmap.cost(1, 3), Some(4));
        assert_eq!(wmap.cost(1, 4), Some(9));
        assert_eq!(wmap.cost(2, 3), Some(10));
        assert_eq!(wmap.cost(2, 4), Some(3));
        assert_eq!(wmap.cost(3, 4), Some(13));
        
        assert_eq!(wmap.costs(), 6+4+9+10+3+13);
        assert_eq!(wmap.avg(), 7.5);
        println!("{:#?}", wmap.avg());
    }

    #[test]
    fn test_2() {
        let s = "\
9
3 4 11
8 2 12
6 5 5
9 6 6
6 1 3
8 3 11
6 4 10
8 7 3";

        let map = make_map(s);
        let wmap = WorldMap::new(&map);
        
        assert_eq!(wmap.avg(), 23.61111111111111);
        println!("{:#?}", wmap.avg());
    }
}
