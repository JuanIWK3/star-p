use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const GRID_SIZE: usize = 8;

fn calc_dist(point1: (usize, usize), point2: (usize, usize)) -> f64 {
    let x1 = point1.0 as f64;
    let y1 = point1.1 as f64;
    let x2 = point2.0 as f64;
    let y2 = point2.1 as f64;

    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

#[derive(Clone, Copy, PartialEq)]
struct Node {
    point: (usize, usize),
    g_score: f64,
    h_score: f64,
}

impl Eq for Node {}

impl Node {
    fn new(point: (usize, usize), g_score: f64, h_score: f64) -> Node {
        Node {
            point,
            g_score,
            h_score,
        }
    }

    fn f_score(&self) -> f64 {
        self.g_score + self.h_score
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.f_score()
                .partial_cmp(&other.f_score())
                .unwrap()
                .reverse(),
        )
    }
}

struct Game {
    board: [[char; GRID_SIZE]; GRID_SIZE],
    player: (usize, usize),
    destination: (usize, usize),
}

impl Game {
    fn new() -> Game {
        Game {
            board: [['-'; GRID_SIZE]; GRID_SIZE],
            player: (0, 0),
            destination: (4, 7),
        }
    }

    fn print_board(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if (i, j) == self.player {
                    print!("P ");
                } else if (i, j) == self.destination {
                    print!("D ");
                } else {
                    print!("{} ", char);
                }
            }
            println!();
        }
        println!();
    }

    fn play(&mut self, path: Vec<(usize, usize)>) {
        for (i, j) in path {
            self.player = (i, j);
            self.print_board();
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    fn get_neighbors(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        let x = point.0 as i32;
        let y = point.1 as i32;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let new_x = x + i;
                let new_y = y + j;

                if new_x >= 0 && new_x < GRID_SIZE as i32 && new_y >= 0 && new_y < GRID_SIZE as i32
                {
                    neighbors.push((new_x as usize, new_y as usize));
                }
            }
        }

        neighbors
    }

    fn a_star(&self) -> Option<Vec<(usize, usize)>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_scores = HashMap::new();

        g_scores.insert(self.player, 0.0);

        open_set.push(Node::new(
            self.player,
            0.0,
            calc_dist(self.player, self.destination),
        ));

        while let Some(current) = open_set.pop() {
            if current.point == self.destination {
                let mut path = vec![current.point];
                let mut node = current;
                while let Some(&prev_point) = came_from.get(&node.point) {
                    path.push(prev_point);
                    node = Node::new(prev_point, 0.0, 0.0);
                }
                path.reverse();
                return Some(path);
            }

            for neighbor in self.get_neighbors(current.point) {
                let tentative_g_score =
                    g_scores[&current.point] + calc_dist(current.point, neighbor);
                if !g_scores.contains_key(&neighbor) || tentative_g_score < g_scores[&neighbor] {
                    g_scores.insert(neighbor, tentative_g_score);
                    let h_score = calc_dist(neighbor, self.destination);
                    open_set.push(Node::new(neighbor, tentative_g_score, h_score));
                    came_from.insert(neighbor, current.point);
                }
            }
        }

        None
    }
}

fn main() {
    let mut game = Game::new();

    game.print_board();

    if let Some(path) = game.a_star() {
        game.play(path);
    } else {
        println!("No path found!");
    }
}
