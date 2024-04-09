use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

struct Game {
    board: [[char; 8]; 8],
    player: (usize, usize),
    barriers: Vec<(usize, usize)>,
    fruit: (usize, usize),
    enemy: (usize, usize),
    destination: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    point: (usize, usize),
    g_cost: usize, // Cost from start node to current node
    h_cost: usize, // Cost from current node to destination node
}

impl Node {
    fn new(point: (usize, usize), g_cost: usize, h_cost: usize) -> Node {
        Node {
            point,
            g_cost,
            h_cost,
        }
    }

    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

// Ordering for the BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost()
            .partial_cmp(&self.f_cost())
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(
    start: (usize, usize),
    goal: (usize, usize),
    barriers: &Vec<(usize, usize)>,
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();

    g_score.insert(start, 0.0);

    open_set.push(Node::new(start, 0, calculate_distance(start, goal)));

    while let Some(current) = open_set.pop() {
        if current.point == goal {
            return Some(reconstruct_path(&came_from, goal));
        }

        closed_set.insert(current.point);

        for neighbor in neighbors(current.point) {
            if closed_set.contains(&neighbor) || barriers.contains(&neighbor) {
                continue;
            }

            let tentative_g_score =
                g_score[&current.point] as usize + calculate_distance(current.point, neighbor);

            if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] as usize {
                came_from.insert(neighbor, current.point);
                g_score.insert(neighbor, tentative_g_score as f64);
                let f_score = tentative_g_score + calculate_distance(neighbor, goal);
                open_set.push(Node::new(neighbor, tentative_g_score, f_score));
            }
        }
    }

    None
}

fn neighbors(point: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = point;
    let mut neighbors = Vec::new();

    for &dx in &[-1, 0, 1] {
        for &dy in &[-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }

            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x >= 0 && new_x < 8 && new_y >= 0 && new_y < 8 {
                neighbors.push((new_x as usize, new_y as usize));
            }
        }
    }

    neighbors
}

fn reconstruct_path(
    came_from: &HashMap<(usize, usize), (usize, usize)>,
    current: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path = vec![current];
    let mut current = current;

    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }

    path.reverse();
    path
}

fn calculate_distance(point1: (usize, usize), point2: (usize, usize)) -> usize {
    let x1 = point1.0 as f64;
    let y1 = point1.1 as f64;
    let x2 = point2.0 as f64;
    let y2 = point2.1 as f64;

    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt().round() as usize
}

impl Game {
    fn new() -> Game {
        Game {
            board: [['-'; 8]; 8],
            player: (0, 0),
            barriers: vec![(0, 2), (1, 2), (2, 2), (3, 2)],
            fruit: (3, 0),
            enemy: (7, 7),
            destination: (0, 7),
        }
    }

    fn print_board(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if (i, j) == self.player {
                    print!("P ");
                } else if self.barriers.contains(&(i, j)) {
                    print!("B ");
                } else if (i, j) == self.fruit {
                    print!("F ");
                } else if (i, j) == self.enemy {
                    print!("E ");
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

    fn get_direction(&mut self, direction: &str) -> (usize, usize) {
        let (x, y) = self.player;

        match direction {
            "u" => {
                // up
                if x > 0 {
                    (x - 1, y)
                } else {
                    (x, y)
                }
            }
            "d" => {
                // down
                if x < 7 {
                    (x + 1, y)
                } else {
                    (x, y)
                }
            }
            "l" => {
                // left
                if y > 0 {
                    (x, y - 1)
                } else {
                    (x, y)
                }
            }
            "r" => {
                // right
                if y < 7 {
                    (x, y + 1)
                } else {
                    (x, y)
                }
            }
            "ul" => {
                // up-left
                if x > 0 && y > 0 {
                    (x - 1, y - 1)
                } else {
                    (x, y)
                }
            }
            "ur" => {
                // up-right
                if x > 0 && y < 7 {
                    (x - 1, y + 1)
                } else {
                    (x, y)
                }
            }
            "dl" => {
                // down-left
                if x < 7 && y > 0 {
                    (x + 1, y - 1)
                } else {
                    (x, y)
                }
            }
            "dr" => {
                // down-right
                if x < 7 && y < 7 {
                    (x + 1, y + 1)
                } else {
                    (x, y)
                }
            }
            _ => (x, y),
        }
    }

    fn move_player(&mut self, direction: &str) {
        let new_player = self.get_direction(direction);

        if !self.check_barrier(new_player) {
            self.player = new_player;
        }
    }

    fn check_collision(&self) -> bool {
        self.player == self.enemy || self.player == self.destination
    }

    fn check_barrier(&self, pos: (usize, usize)) -> bool {
        self.barriers.contains(&pos)
    }

    fn check_fruit(&self) -> bool {
        self.player == self.fruit
    }

    fn check_win(&self) -> bool {
        self.player == self.destination
    }

    fn check_loss(&self) -> bool {
        self.player == self.enemy || self.barriers.contains(&self.player)
    }

    fn remove_barriers(&mut self) {
        self.barriers.clear();
    }

    fn play(&mut self) {
        self.print_board();

        loop {
            let mut direction = String::new();

            println!("Enter a direction: ");
            std::io::stdin().read_line(&mut direction).unwrap();

            let direction = direction.trim();

            self.move_player(direction);

            if self.check_fruit() {
                println!("You ate the fruit! Barriers removed.");
                self.remove_barriers();
                // remove fruit from game
                self.fruit = (8, 8);
            }

            self.print_board();

            if self.check_collision() {
                if self.check_win() {
                    println!("You win!");
                } else if self.check_loss() {
                    println!("You lose!");
                }
                break;
            }
        }
    }
}

fn main() {
    let mut game = Game::new();

    let path = a_star(game.player, game.destination, &game.barriers).unwrap();

    for point in path {
        game.player = point;
        game.print_board();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
