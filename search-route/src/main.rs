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
    prev  : Option<Pos>,
}

struct GameMap {
    w     : usize,
    h     : usize,
    start : Pos,
    goal  : Pos,
    nodes : HashMap<Pos, Rc<RefCell<Node>>>,
}

impl GameMap {
    fn new(s: &str) -> GameMap {
        let mut lines = s.lines();

        let wh: Vec<String> = lines.next().unwrap().split_whitespace()
            .map(|e| e.parse().ok().unwrap()).collect();

        let rmap: Vec<Vec<String>> = lines.map(|l| l.split_whitespace()
                                               .map(|e| e.parse().ok().unwrap()).collect()).collect();

        let w: usize = wh[0].parse::<usize>().unwrap();
        let h: usize = wh[1].parse::<usize>().unwrap();

        let mut start = Pos {x: 0, y: 0};
        let mut goal = Pos {x: 0, y: 0};
        let mut nodes: HashMap<Pos, Rc<RefCell<Node>>> = HashMap::new();
        for (i, l) in rmap.iter().enumerate() {
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
            w,
            h,
            start,
            goal,
            nodes,
        }
    }

    fn goal_node(&mut self) -> Option<&Rc<RefCell<Node>>> {
        self.parse_nodes(self.nodes[&self.start].clone());
        self.nodes.get(&self.goal)
    }

    fn parse_nodes(&mut self, cur_node: Rc<RefCell<Node>>) {
        let next_poses = self.next_poses(&cur_node);
        let cur_node = cur_node.borrow();

        for next_pos in next_poses {
            if let Some(next_node) = self.nodes.get(&next_pos) {
                let mut next_node = next_node.borrow_mut();
                if next_node.step < 0 || next_node.step > cur_node.step + 1 {
                    next_node.step = cur_node.step + 1;
                    next_node.prev = Some(cur_node.pos);
                }
            }

            if let Some(next_node) = self.nodes.get(&next_pos) {
                self.parse_nodes(next_node.clone());
            }
        }
    }

    fn next_poses(&self, cur_node: &Rc<RefCell<Node>>) -> Vec<Pos> {
        let mut nexts: Vec<Pos> = Vec::new();
        let cb = cur_node.borrow();
        let cur_pos = cb.pos;

        for p in [[0,1],[0,-1],[1,0],[-1,0]].iter() {
            if (cur_pos.x == 0 && p[0] < 0) || (cur_pos.y == 0 && p[1] < 0) {
                continue;
            }

            let np = Pos {
                x: ((cur_pos.x as i32) + p[0]) as usize,
                y: ((cur_pos.y as i32) + p[1]) as usize,
            };

            if self.is_valid(&np, &cb) {
                nexts.push(np);
            }
        }

        nexts
    }


//    fn parse_nodes(&mut self, cur_node: &Node) {
//        let next_nodes_ = self.next_nodes(&cur_node);
//
//        for next_node in next_nodes_ {            
//            if (*next_node).step < 0 || (*next_node).step > cur_node.step + 1 {
//                (*next_node).step = cur_node.step + 1;
//                //next_node.prev = Some(cur_node); @@@ ???
//            }
//
//            self.parse_nodes(&next_node);
//        }
//    }
//
//    fn next_nodes(&mut self, cur_node: &Node) -> Vec<Node> {
//        let mut nexts: Vec<Node> = Vec::new();
//        
//        for p in [[0,1],[0,-1],[1,0],[-1,0]].iter() {
//            if (cur_node.pos.x == 0 && p[0] < 0) || (cur_node.pos.y == 0 && p[1] < 0) {
//                continue;
//            }
//
//            let np = Pos {
//                x: ((cur_node.pos.x as i32) + p[0]) as usize,
//                y: ((cur_node.pos.y as i32) + p[1]) as usize,
//            };
//
//            if self.is_valid(&np, &cur_node) {
//                let next_node = self.nodes.entry(np).or_insert(Node {pos: np, step: -1, prev: None});
////                let b = Box::new(Node {pos: np, step: -1, prev: None});
////                let next_node = self.nodes.entry(np).or_insert(b);
//                nexts.push(next_node.clone());
//            }
//        }
//        
//        nexts
//    }

    fn is_valid(&self, pos: &Pos, cur_node: &Node) -> bool {
        match self.nodes.get(pos) {
            Some(node) => {
                if node.borrow().value == "1" {
                    return false;
                }
            },
            None => {
                return false;
            }
        }

        if let Some(p) = &cur_node.prev {
            if p.x == pos.x && p.y == pos.y {
                return false;
            }
        }

        true
    }
}

impl fmt::Debug for GameMap {
    // このトレイトは`fmt`が想定通りのシグネチャであることを要求します。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{");
        write!(f, "w: {}, ", self.w);
        write!(f, "h: {}, ", self.h);
        write!(f, "start: {:?}, ", self.start);
        write!(f, "goal: {:?}, ", self.goal);
        writeln!(f, "node: {{");
        for (_, v) in &self.nodes {
            write!(f, "{{ ");
            write!(f, "{:?}, ", v.borrow().pos);
            write!(f, "{:?}, ", v.borrow().value);
            write!(f, "step: {:?}, ", v.borrow().step);
            write!(f, "prev: {:?}", v.borrow().prev);
            writeln!(f, " }}");
        }

        //nodes : HashMap<Pos, Rc<RefCell<Node>>>,
        write!(f, "}}")
    }
}

fn main() {
//    let s = ""; //@@@
//
//    let mut gmap = GameMap::new(s);
//    //println!("{:?}", gmap);
//
//    let goal = gmap.goal_node();
//    //println!("{:?}", goal);
//    //print_route(&goal);
//    
//    match goal {
//        Some(node) => {
//            println!("{}", node.step);
//        },
//        None => println!("Fail")
//    }
}

//fn read<T: std::str::FromStr>() -> T {
//    let mut s = String::new();
//    std::io::stdin().read_line(&mut s).ok();
//    s.trim().parse().ok().unwrap()
//}
//
//fn read_vec<T: std::str::FromStr>() -> Vec<T> {
//    read::<String>().split_whitespace()
//        .map(|e| e.parse().ok().unwrap()).collect()
//}
//
//fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
//    (0..n).map(|_| read_vec()).collect()
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_routes() {
        let s = "\
4 5
0 s 0 1
0 0 1 0
0 1 1 0
0 0 1 g
0 0 0 0";

        let mut gmap = GameMap::new(s);
        println!("{:?}", gmap);

        let goal = gmap.goal_node();
        println!("{:?}", gmap);

//        //println!("{:?}", goal);
//        //print_route(&goal);
//
//        assert_eq!(goal.pos, Pos {x: 3, y: 3});
//        assert_eq!(goal.step, 9);
    }
}
