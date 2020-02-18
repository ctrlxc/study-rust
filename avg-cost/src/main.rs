use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;
use std::thread;
use std::usize;
use std::sync::mpsc;

#[derive(Debug, Clone, Copy)]
struct Pos {
    no1: usize,
    no2: usize,
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
struct State {
    no  : usize,
    cost: usize,
    prev: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.no.cmp(&other.no))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct WorldMap {
    nodes: HashMap<usize, Vec<usize>>,
    costs: HashMap<Pos, usize>,
    visited: Arc<Mutex<HashMap<Pos, usize>>>,
    all: Arc<Mutex<usize>>,
}

impl WorldMap {
    fn new(map: &Vec<Vec<String>>) -> WorldMap {
        let mut nodes: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut costs: HashMap<Pos, usize> = HashMap::new();

        for (_, l) in map.iter().enumerate() {
            let no1 = l[0].parse::<usize>().unwrap();
            let no2 = l[1].parse::<usize>().unwrap();
            let cost = l[2].parse::<usize>().unwrap();

            nodes.entry(no1).or_insert(Vec::new()).push(no2);
            nodes.entry(no2).or_insert(Vec::new()).push(no1);

            let pos = WorldMap::make_pos(no1, no2);
            costs.entry(pos).or_insert(cost);
        }

        WorldMap {
            nodes,
            costs,
            visited: Arc::new(Mutex::new(HashMap::new())),
            all: Arc::new(Mutex::new(0)),
        }
    }

    fn make_pos(no1: usize, no2: usize) -> Pos {
        Pos {no1: std::cmp::min(no1, no2), no2: std::cmp::max(no1, no2)}
    }

    fn calc(&self, start_no: usize) {
        let mut status: Vec<State> = Vec::new();
        status.push(State {no: start_no, cost: 0, prev: 0});

        while let Some(State {no, cost, prev}) = status.pop() {
            if no > start_no {
                self.visit(&WorldMap::make_pos(start_no, no), cost);
            }

            let nexts = self.nodes.get(&no).unwrap().iter().filter(|x| **x != prev);

            for next_no in nexts {
                let pos = WorldMap::make_pos(no, *next_no);
                let next_cost = self.costs.get(&pos).unwrap();
                status.push(State {no: *next_no, cost: (cost + *next_cost), prev: no});
            }
        }
    }

    fn visit(&self, pos: &Pos, cost: usize) {
        #[cfg(test)]
        {
            println!("pos: {:?}, cost {}", pos, cost);
        //    let mut visited = self.visited.lock().unwrap();
        //    visited.insert(*pos, cost);
        }

        *self.all.lock().unwrap() += cost;
    }

    fn cost(&self) -> usize {
        *self.all.lock().unwrap()
    }

    fn avg(&self) -> f64 {
        let cost = self.cost();
        let len = self.nodes.len();
        let num = (len * (len-1)) / 2;
        (cost as f64) / (num as f64)
    }
}


fn calc(wmap: &Arc<WorldMap>, thread: usize) {
    let keys: Vec<usize> = wmap.nodes.keys().map(|x| *x).collect();
    let chunk_size = (keys.len() as f64 / thread as f64).ceil() as usize;
    
    let mut handles = Vec::new();

    for chunk in keys.chunks(chunk_size) {
        let wmap = wmap.clone();
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            for c in chunk {
                wmap.calc(c);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join();
    }
}

fn main() {
    let map = read_map();
    let wmap = Arc::new(WorldMap::new(&map));
    calc(&wmap, 8);
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
        let wmap = Arc::new(WorldMap::new(&map));
        calc(&wmap, 8);
        assert_eq!(wmap.cost(), 6+4+9+10+3+13);
        assert_eq!(wmap.avg(), 7.5);
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
        let wmap = Arc::new(WorldMap::new(&map));
        calc(&wmap, 8);
        assert_eq!(wmap.cost(), 850);
        assert_eq!(wmap.avg(), 23.61111111111111);
    }

    #[test]
    fn test_big_auto() {
        let size: usize = 100_000;
        let mut map: Vec<Vec<String>> = Vec::new();

        let mut cost: usize = 0;
        for i in 0..size {
            let mut v = Vec::new();
            v.push((i+1).to_string());
            v.push((i+2).to_string());
            v.push((i+1).to_string());
            map.push(v);
        }

        let wmap = Arc::new(WorldMap::new(&map));
        //println!("hoge");
        calc(&wmap, 8);
        assert_eq!(wmap.cost(), 8670850);
        assert_eq!(wmap.avg(), 1717.0);
    }
}
