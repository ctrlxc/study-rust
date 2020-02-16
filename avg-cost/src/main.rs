use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
//use std::fmt;
use std::usize;

#[derive(Eq, PartialEq)]
struct Node {
    no    : usize,
    cost  : usize,
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
}

impl WorldMap {
    fn new(map: &Vec<Vec<String>>) -> WorldMap {
        let mut nodes: HashMap<usize, Vec<Node>> = HashMap::new();

        for (_, l) in map.iter().enumerate() {
            let no1 = l[0].parse::<usize>().unwrap();
            let no2 = l[1].parse::<usize>().unwrap();
            let cost = l[2].parse::<usize>().unwrap();

            nodes.entry(no1).or_insert(Vec::new()).push(Node {no: no2, cost});
            nodes.entry(no2).or_insert(Vec::new()).push(Node {no: no1, cost});
        }

        WorldMap {
            nodes,
        }
    }

    fn cost(&self, start: usize, goal: usize) -> Option<usize> {
        let mut costs: Vec<_> = (0..self.nodes.len()).map(|_| usize::MAX).collect();
        costs[start-1] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Node {no: start, cost: 0});

        while let Some(Node {no, cost}) = heap.pop() {
            if no == goal {
                return Some(cost);
            }

            if cost > costs[no-1] {
                continue;
            }

            let nexts = self.nodes.get(&no).unwrap();

            for next in nexts.iter() {
                let nx = Node {no: next.no, cost: cost + next.cost};

                if costs[nx.no-1] > nx.cost {
                    costs[nx.no-1] = nx.cost;
                    heap.push(nx);
                }
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
