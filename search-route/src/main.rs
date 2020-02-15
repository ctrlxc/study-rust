//search-route.rs
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::fmt;

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
    prev  : Option<Weak<RefCell<Node>>>,
    done  : bool,
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
                let mut node = Node {pos, value: c.clone(), step: -1, prev: None, done: false};

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

    fn goal(&self) -> Option<&Rc<RefCell<Node>>> {
        self.nodes.get(&self.goal)
    }

    fn calc(&self) {
        while let Some(node) = self.get_calc_node() {
            self.calc_node(&node);
        }
    }

    fn get_calc_node(&self) -> Option<&Rc<RefCell<Node>>> {
        for (_, v) in &self.nodes {
            if self.can_calc_node(&v.borrow()) {
                return Some(&v);
            }
        }

        None
    }
    
    fn can_calc_node(&self, node: &Node) -> bool {
        if node.done {
            return false;
        }

        if node.step < 0 {
            return false;
        }

        if node.value == "1" {
            return false;
        }

        true
    }
      
    fn calc_node(&self, node: &Rc<RefCell<Node>>) {
        if node.borrow().done {
            return;
        }

        node.borrow_mut().done = true;

        let next_nodes = self.next_nodes(&node.borrow());

        for next_node in next_nodes.iter() {
            {
                let nx = next_node.upgrade().unwrap();
                let mut next_node = nx.borrow_mut();
    
                if next_node.step < 0 || next_node.step > node.borrow().step + 1 {
                    next_node.step = node.borrow().step + 1;
                    next_node.prev = Some(Rc::downgrade(&node));
                }
            }
        }
    }

    fn _parse(&self) {
        self._parse_node(&self.nodes.get(&self.start).unwrap());
    }

    fn _parse_node(&self, cur_node: &Rc<RefCell<Node>>) {
        let next_nodes = self.next_nodes(&cur_node.borrow());

        for next_node in next_nodes.iter() {
            {
                let nx = next_node.upgrade().unwrap();
                let mut next_node = nx.borrow_mut();
    
                if next_node.step < 0 || next_node.step > cur_node.borrow().step + 1 {
                    next_node.step = cur_node.borrow().step + 1;
                    next_node.prev = Some(Rc::downgrade(&cur_node));
                }
            }

            self._parse_node(&next_node.upgrade().unwrap());
        }
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
            let p = p.upgrade().unwrap();
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

    fn print_route(&self, node: &Rc<RefCell<Node>>) {
        println!("{:#?}", *node.borrow());
        if let Some(p) = &node.borrow().prev {
            self.print_route(&p.upgrade().unwrap());
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{")?;
        write!(f, "pos: {:?}, ", self.pos)?;
        write!(f, "value: {}, ", self.value)?;
        write!(f, "step: {}, ", self.step)?;

        match &self.prev {
            Some(p) => {
                let p = p.upgrade().unwrap();
                write!(f, "prev-pos: {:?}", p.borrow().pos)?;
            },
            None => {
                write!(f, "prev-pos: {{None}}")?;
            },
        };
        
        write!(f, "}}")
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

    gmap.calc();
    //println!("{:?}", gmap);
        
    let goal = gmap.goal();

    if let Some(goal) = goal {
        //gmap.print_route(goal);

        let g = goal.borrow();
        if g.step > 0 {
            println!("{}", g.step);
        }
        else {
            println!("Fail");
        }
    }
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
    fn test_goal() {
        let s = "\
4 5
0 s 0 1
0 0 1 0
0 1 1 0
0 0 1 g
0 0 0 0";

        let map = make_map(s);
        let gmap = GameMap::new(&map);
        gmap.calc();
        println!("{:?}", gmap);
        
        let goal = gmap.goal();

        if let Some(goal) = goal {
            gmap.print_route(goal);
        }

        let g = goal.unwrap().borrow();
        assert_eq!(g.pos, Pos {x: 3, y: 3});
        assert_eq!(g.step, 9);
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
        gmap.calc();
        println!("{:?}", gmap);
        
        let goal = gmap.goal();

        if let Some(goal) = goal {
            gmap.print_route(goal);
        }

        let g = goal.unwrap().borrow();
        assert_eq!(g.pos, Pos {x: 3, y: 3});
        assert_eq!(g.step, -1);
    }
    
}
