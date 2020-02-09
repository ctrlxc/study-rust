//search-route.rs

#[derive(Debug, Clone, Copy)]
struct Pos {
  x: usize,
  y: usize,
}

fn in_pos(x: usize, y: usize, arr: &Vec<Pos>) -> bool {
  for a in arr {
    if a.x == x && a.y == y {
       return true;
    }
  }

  return false;
}

struct GameMap {
   w: usize,
   h: usize,
   rmap: Vec<Vec<String>>,
   start: Pos,
   goal: Pos,
   oks: Vec<Pos>,
   ngs: Vec<Pos>,
}

impl GameMap {
  fn is_valid(&self, x: usize, y: usize, cur_route: &Vec<Pos>) -> bool {
    if x >= self.w || y >= self.h  {
       return false;
    }
  
    if in_pos(x, y, &cur_route)  {
       return false;
    }
  
    if in_pos(x, y, &self.ngs)  {
       return false;
    }
  
    true
  }

  fn is_goal(&self, x: usize, y: usize) -> bool {
    if x == self.goal.x && y == self.goal.y {
      return true;
    }

    false
  }
  
  fn is_goal_pos(&self, pos: &Pos) -> bool {
    self.is_goal(pos.x, pos.y)
  }

  fn nexts(&self, cur_route: &Vec<Pos>) -> Vec<Pos> {
    let cur_pos = cur_route.last().unwrap();

    let x = cur_pos.x;
    let y = cur_pos.y;
  
    let pat: [[i32; 2]; 4] = [[0,1],[0,-1],[1,0],[-1,0]];
  
    let mut nexts: Vec<Pos> = Vec::new();
  
    for p in pat.iter() {
      if (x == 0 && p[0] < 0) || (y == 0 && p[1] < 0) {
        continue;
      }

      let x2 = (x as i32) + p[0];
      let y2 = (y as i32) + p[1];

      let x2 = x2 as usize;
      let y2 = y2 as usize;

      if self.is_goal(x2, y2) {
        return vec![self.goal.clone()];
      }

      if self.is_valid(x2, y2, &cur_route) {
        nexts.push(Pos {x: x2, y: y2});
      }
    }
  
    nexts
  }

  fn find_routes(&self, cur_route: &Vec<Pos>, goals: &mut Vec<Vec<Pos>>) {
    let nexts = self.nexts(cur_route);
  
    if nexts.len() == 1 && self.is_goal_pos(&nexts[0]) {
      goals.push(cur_route.clone());
      return;
    }
  
    for n in nexts {
      let mut new_route = cur_route.clone();
      new_route.push(n);
      self.find_routes(&new_route, goals);
    }
  }
}

fn init() -> (Vec<String>, Vec<Vec<String>>) {
   let wh = read_vec::<String>();
   let w = &wh[0];
   let h = &wh[1];
   let rmap = read_vec2::<String>(h.parse::<u32>().unwrap());

   (wh, rmap)
}

fn print_route(route: &Vec<Pos>) {
  for (i, r) in route.iter().enumerate() {
    print!("[{}]({}, {}) -> ", i, r.x, r.y);
  }

  println!("g");
}

fn print_route_all(routes: &Vec<Vec<Pos>>) {
  for (i, r) in routes.iter().enumerate() {
    print!("[{}] :: ", i);
    print_route(&r);
  }
}

fn find_routes(wh: Vec<String>, rmap: Vec<Vec<String>>) -> Vec<Vec<Pos>> {
   let w = &wh[0];
   let h = &wh[1];

   let mut gmap: GameMap = GameMap {
      w: w.parse::<usize>().unwrap(),
      h: h.parse::<usize>().unwrap(),
      rmap,
      start: Pos {x: 0, y: 0},
      goal: Pos {x: 0, y: 0},
      oks: Vec::<Pos>::new(),
      ngs: Vec::<Pos>::new(),
   };

   for (i, l) in gmap.rmap.iter().enumerate() {
      for (j, c) in l.iter().enumerate() {
        let pos = Pos {x: j, y: i};
        
        if c == "s" {
          gmap.start = pos;
        }
        else if c == "g" {
          gmap.goal = pos;
        }
        else if c == "1" {
          gmap.ngs.push(pos);
        }
        else if c == "0" {
          gmap.oks.push(pos);
        }
      }
   }

   let cur_route: Vec<Pos> = vec![gmap.start];
   let mut goals: Vec<Vec<Pos>> = vec![];
   
   gmap.find_routes(&cur_route, &mut goals);

   goals
}

fn min_steps(routes: &Vec<Vec<Pos>>) -> usize {
  let mut min: usize = 0; 
  for r in routes.iter() {
    if min == 0 || r.len() < min {
      min = r.len();
    }
  }

  min
}

fn main() {
   let (wh, rmap) = init();
   let goals = find_routes(wh, rmap);
   let min = min_steps(&goals);

   if min == 0 {
    println!("Fail");
    return;
   }

   println!("{}", min);
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

    fn make(str: &str) -> (Vec<String>, Vec<Vec<String>>) {
      let mut lines = str.lines();

      let wh = lines.next().unwrap().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect();

      let rmap = lines.map(|l| l.split_whitespace()
          .map(|e| e.parse().ok().unwrap()).collect()).collect();

      (wh, rmap)
    }

    #[test]
    fn test_find_routes() {
      let inp = "\
4 5
0 s 0 1
0 0 1 0
0 1 1 0
0 0 1 g
0 0 0 0";

      let (wh, rmap) = make(inp);
      println!("{:?}", wh);
      println!("{:?}", rmap);
      let goals = find_routes(wh, rmap);

      println!("{:#?}", goals);
      print_route_all(&goals);

      assert_eq!(min_steps(&goals), 9);
    }
}
