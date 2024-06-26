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
    barriers: Vec<(usize, usize)>,
    powerup: (usize, usize),
    has_powerup: bool,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [['-'; GRID_SIZE]; GRID_SIZE],
            player: (0, 0),
            destination: (4, 7),
            barriers: vec![
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (4, 2),
                (5, 2),
                (6, 2),
                (7, 2),
            ],
            powerup: (5, 0),
            has_powerup: false,
        }
    }

    fn print_board(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if (i, j) == self.player {
                    print!("P ");
                } else if self.barriers.contains(&(i, j)) {
                    print!("x ");
                } else if (i, j) == self.destination {
                    print!("D ");
                } else if (i, j) == self.powerup {
                    print!("O ");
                } else {
                    print!("{} ", char);
                }
            }
            println!();
        }
        println!();
    }

    fn play(&mut self, instructions: Vec<(usize, usize)>) {
        for (i, j) in instructions {
            if !self.has_powerup && self.powerup == (i, j) {
                self.has_powerup = true;
            }
            if self.has_powerup {
                // Clear barriers if the player has the powerup
                self.barriers.clear();
            }
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

                // Check if the neighbor position is within the grid boundaries
                if new_x >= 0 && new_x < GRID_SIZE as i32 && new_y >= 0 && new_y < GRID_SIZE as i32
                {
                    let neighbor = (new_x as usize, new_y as usize);

                    // Check if the neighbor position is not a barrier or the player has the power-up
                    if !self.barriers.contains(&neighbor) || self.has_powerup {
                        neighbors.push(neighbor);
                    }
                }
            }
        }

        neighbors
    }

    fn a_star(&mut self) -> Vec<(usize, usize)> {
        // A* from player to destination directly
        let path_to_destination = self.a_star_path(self.player, self.destination);
        // A* from player to powerup to destination
        let path_to_powerup = self.a_star_path(self.player, self.powerup);
        let path_from_powerup_to_destination = self.a_star_path(self.powerup, self.destination);

        match path_to_powerup {
            Some(path_to_powerup) => match path_from_powerup_to_destination {
                Some(path_from_powerup_to_destination) => {
                    let mut path = path_to_powerup.clone();
                    path.extend(path_from_powerup_to_destination);
                    path
                }
                None => path_to_destination.expect("No path found"),
            },
            None => path_to_destination.expect("No path found"),
        }
    }

    fn a_star_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_scores = HashMap::new();

        g_scores.insert(start, 0.0);

        open_set.push(Node::new(start, 0.0, calc_dist(start, goal)));

        while let Some(current) = open_set.pop() {
            if current.point == goal {
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
                    let h_score = calc_dist(neighbor, goal);
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

    let path = game.a_star();

    game.play(path);
}
