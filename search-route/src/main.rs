//search-route.rs
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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
    pos: Pos,
    step: i32,
    prev: Option<Box<Node>>,
}

struct GameMap {
    w     : usize,
    h     : usize,
    hmap  : HashMap<Pos, String>,
    start : Pos,
    goal  : Pos,
    nodes : HashMap<Pos, Node>,
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
        let mut hmap: HashMap<Pos, String> = HashMap::new();
        for (i, l) in rmap.iter().enumerate() {
            for (j, c) in l.iter().enumerate() {
                let pos = Pos {x: j, y: i};

                hmap.insert(pos, c.to_string()); //@@@ to_string() naze iru?

                if c == "s" {
                    start = pos;
                }
                else if c == "g" {
                    goal = pos;
                }
            }
        }

        GameMap {
            w,
            h,
            hmap,
            start,
            goal,
            nodes: HashMap::new(),
        }
    }

    fn goal_node(&mut self) -> Option<&Node> {
        if self.nodes.len() == 0 {
//@@@            let start_node = self.nodes.insert(self.start, Node {pos: self.start, step: 0, prev: None}).unwrap();
//@@@            self.parse_nodes(&start_node);
        }

        self.nodes.get(&self.goal)
    }

    fn parse_nodes(&mut self, cur_node: &Node) {
        let next_poses = self.next_poses(&cur_node);

        for next_pos in next_poses {
            let next_node = self.nodes.entry(next_pos).or_insert(Node {pos: next_pos, step: -1, prev: None});
            
            if (*next_node).step < 0 || (*next_node).step > cur_node.step + 1 {
                (*next_node).step = cur_node.step + 1;
                //next_node.prev = Some(cur_node); @@@ ???
            }

            self.parse_nodes(next_node);
        }
    }

    fn next_poses(&mut self, cur_node: &Node) -> Vec<Pos> {
        let mut nexts: Vec<Pos> = Vec::new();
        
        for p in [[0,1],[0,-1],[1,0],[-1,0]].iter() {
            if (cur_node.pos.x == 0 && p[0] < 0) || (cur_node.pos.y == 0 && p[1] < 0) {
                continue;
            }

            let np = Pos {
                x: ((cur_node.pos.x as i32) + p[0]) as usize,
                y: ((cur_node.pos.y as i32) + p[1]) as usize,
            };

            if self.is_valid(&np, &cur_node) {
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
        match self.hmap.get(pos) {
            Some(v) => {
                if v == "1" {
                    return false;
                }
            },
            None => {
                return false;
            }
        }

        if let Some(p) = &cur_node.prev {
            if p.pos.x == pos.x && p.pos.y == pos.y {
                return false;
            }
        }

        true
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
        //println!("{:?}", gmap);

//        let goal = gmap.goal_node();
//        //println!("{:?}", goal);
//        //print_route(&goal);
//
//        assert_eq!(goal.pos, Pos {x: 3, y: 3});
//        assert_eq!(goal.step, 9);
    }
}
