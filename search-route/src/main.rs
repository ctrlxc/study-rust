//search-route.rs
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;
use std::cmp::Ordering;
use std::usize;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Pos {}

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> Ordering {
        other.x.cmp(&self.x)
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Pos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

struct Node {
    pos   : Pos,
    value : String,
    step  : i32,
    prev  : Option<Pos>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{")?;
        write!(f, "pos: {:?}, ", self.pos)?;
        write!(f, "value: {}, ", self.value)?;
        write!(f, "step: {}, ", self.step)?;
        write!(f, "prev: {:?}, ", self.prev)?;
        write!(f, "}}")
    }
}

#[derive(Eq, PartialEq)]
struct State {
    pos   : Pos,
    step  : i32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.step.cmp(&self.step)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct GameMap {
    start : Pos,
    goal  : Pos,
    nodes : HashMap<Pos, Rc<RefCell<Node>>>,
}

impl GameMap {
    fn new(map: &Vec<Vec<String>>) -> GameMap {
        let mut start = Pos {x: 0, y: 0};
        let mut goal = Pos {x: 0, y: 0};
        let mut nodes: HashMap<Pos, Rc<RefCell<Node>>> = HashMap::new();

        for (i, l) in map.iter().enumerate() {
            for (j, c) in l.iter().enumerate() {
                let pos = Pos {x: j, y: i};
                let mut node = Node {pos, value: c.clone(), step: -1, prev: None};

                if c == "s" {
                    start = pos;
                    node.step = 0;
                }
                else if c == "g" {
                    goal = pos;
                }

                nodes.insert(pos, Rc::new(RefCell::new(node)));
            }
        }

        GameMap {
            start,
            goal,
            nodes,
        }
    }

    fn clear(&self) {
        for (_, v) in &self.nodes {
            v.borrow_mut().step = -1;
        }
    }

    fn calc(&self) -> Option<i32> {
        let mut heap = BinaryHeap::new();
        heap.push(State {step: 0, pos: self.start.clone()});

        while let Some(State {step, pos}) = heap.pop() {
            if pos == self.goal {
                return Some(step);
            }

            let node = self.nodes.get(&pos).unwrap();

            if step > node.borrow().step {
                continue;
            }

            for next_node in self.next_nodes(&node.borrow()).iter() {
                let nx = next_node.upgrade().unwrap();
                let mut next_node = nx.borrow_mut();
                let next_step = step + 1;
                
                if next_node.step < 0 || next_node.step > next_step {
                    next_node.step = next_step;
                    next_node.prev = Some(pos);

                    heap.push(State { step: next_step, pos: next_node.pos.clone()});
                }
            }
        }
    
        None
    }
    
    fn next_nodes(&self, cur_node: &Node) -> Vec<Weak<RefCell<Node>>> {
        let mut next_nodes: Vec<Weak<RefCell<Node>>> = Vec::new();
        let cur_pos = &cur_node.pos;

        for p in [[0,1],[0,-1],[1,0],[-1,0]].iter() {
            if (cur_pos.x == 0 && p[0] < 0) || (cur_pos.y == 0 && p[1] < 0) {
                continue;
            }

            let np = Pos {
                x: ((cur_pos.x as i32) + p[0]) as usize,
                y: ((cur_pos.y as i32) + p[1]) as usize,
            };

            if let Some(node) = self.get_valid_node(&np, &cur_node) {
                next_nodes.push(Rc::downgrade(&node));
            }
        }

        next_nodes
    }

    fn get_valid_node(&self, pos: &Pos, cur_node: &Node) -> Option<&Rc<RefCell<Node>>> {
        if let Some(p) = &cur_node.prev {
            let p = self.nodes.get(&p).unwrap();
            let p = p.borrow();
            if p.pos == *pos {
                return None
            }
        }

        match self.nodes.get(pos) {
            Some(node) => {
                let n = node.borrow();

                if n.value == "1" {
                    None
                }
                else {
                    Some(&node)
                }
            },
            None => None,
        }
    }

    fn print_route(&self, pos: &Pos) {
        let node = self.nodes.get(&pos).unwrap();
        println!("{:?}", node.borrow().pos);
        if let Some(p) = &node.borrow().prev {
            self.print_route(&p);
        }
    }
}

impl fmt::Debug for GameMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameMap {{")?;
        write!(f, "start: {:?}, ", self.start)?;
        write!(f, "goal: {:?}, ", self.goal)?;

        writeln!(f, "nodes: {{")?;
        for (_, v) in &self.nodes {
            writeln!(f, "  {:?}, ", v.borrow())?;
        }
        write!(f, "}}")?;

        write!(f, "}}")
    }
}

fn main() {
    let map = read_map();
    let gmap = GameMap::new(&map);
    let step = gmap.calc();

    match step {
        Some(step) => {
            println!("{}", step);
        },
        None => {
            println!("Fail");
        }
    };
}

fn read_map() -> Vec<Vec<String>> {
    let wh = read_vec::<String>();
    let h = &wh[1];
    let map = read_vec2::<String>(h.parse::<u32>().unwrap());

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

        let _wh: Vec<String> = lines.next().unwrap().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect();

        let map: Vec<Vec<String>> = lines.map(|l| l.split_whitespace()
                                              .map(|e| e.parse().ok().unwrap()).collect()).collect();

        map
    }
    
    #[test]
    fn test_normal_goal() {
        let s = "\
4 5
0 s 0 1
0 0 1 0
0 1 1 0
0 0 1 g
0 0 0 0";

        let map = make_map(s);
        let gmap = GameMap::new(&map);
        let step = gmap.calc();
        assert_eq!(step, Some(9));

        gmap.print_route(&gmap.goal);
    }

    #[test]
    fn test_small_x2y1_goal() {
        let s = "\
2 1
g s";

        let map = make_map(s);
        let gmap = GameMap::new(&map);
        let step = gmap.calc();
        assert_eq!(step, Some(1));

        gmap.print_route(&gmap.goal);
    }

    #[test]
    fn test_small_x1y2_goal() {
        let s = "\
1 2
g
s";
        let map = make_map(s);
        let gmap = GameMap::new(&map);
        let step = gmap.calc();
        assert_eq!(step, Some(1));

        gmap.print_route(&gmap.goal);
    }
    
    #[test]
    fn test_nogoal() {
        let s = "\
4 4
0 s 0 1
1 0 0 0
0 1 1 1
0 0 0 g";

        let map = make_map(s);
        let gmap = GameMap::new(&map);
        let step = gmap.calc();
        assert_eq!(step, None);

        gmap.print_route(&gmap.goal);
    }

    #[test]
    fn test_big_goal() {
        let size: usize = 100;
        let mut map: Vec<Vec<String>> = Vec::new();

        for i in 0..size {
            map.push(Vec::new());
            for _ in 0..size {
                map[i].push("0".to_string());
            }
        }

        map[0][0] = "s".to_string();
        map[size-1][size-1] = "g".to_string();

        let gmap = GameMap::new(&map);
        let step = gmap.calc();
        assert_eq!(step, Some((size*2-2) as i32));

        gmap.print_route(&gmap.goal);
    }
}
