use rand::distributions::{IndependentSample, Range};
use std::collections::HashSet;

use util::Pop;

#[derive(Clone)]
pub struct Maze {
    pub walls: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl ::std::fmt::Display for Maze {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "\n")?;
        for j in 0..self.height {
            for i in 0..self.width {
                if self.walls[i][j] {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")
    }
}

#[allow(unused)]
impl Maze {
    pub fn check(&self) {
        assert_eq!(self.walls.len(), self.width);
        for column in &self.walls {
            assert_eq!(column.len(), self.height);
        }
    }

    pub fn full(&self) -> bool {
        for wall in self.walls.iter().flat_map(|column| column.iter()) {
            if !wall {
                return false;
            }
        }
        true
    }

    /// Remove the circle of the maze
    pub fn reduce(&mut self, x: usize) {
        for _ in 0..x {
            self.walls.remove(0);
            self.walls.pop().unwrap();
        }

        for column in &mut self.walls {
            for _ in 0..x {
                column.remove(0);
                column.pop().unwrap();
            }
        }

        self.width -= x * 2;
        self.height -= x * 2;
    }

    /// Create a wall that circle the maze
    pub fn circle(&mut self) {
        self.walls[0] = (0..self.width).map(|_| true).collect();
        self.walls[self.width - 1] = (0..self.width).map(|_| true).collect();
        for column in &mut self.walls {
            column[0] = true;
            column[self.height - 1] = true;
        }
    }

    fn compute_zones(&self, corridor: bool) -> Vec<Vec<(usize, usize)>> {
        let mut unvisited = HashSet::new();
        for i in 0..self.width {
            for j in 0..self.height {
                if !self.walls[i][j] {
                    // the maze must be circled
                    assert!(i != 0 && i != self.width-1);
                    assert!(j != 0 && j != self.height-1);
                    unvisited.insert((i, j));
                }
            }
        }

        let mut to_visit = HashSet::new();
        let mut zones = Vec::new();

        while let Some(cell) = unvisited.pop() {
            let mut zone = Vec::new();
            to_visit.insert(cell);

            while let Some(cell) = to_visit.pop() {
                let i = cell.0;
                let j = cell.1;

                let is_corridor = (self.walls[i-1][j] && self.walls[i+1][j])
                    || (self.walls[i][j-1] && self.walls[i][j+1])
                    || (self.walls[i-1][j] && self.walls[i][j-1] && self.walls[i+1][j+1])
                    || (self.walls[i-1][j] && self.walls[i][j+1] && self.walls[i+1][j-1])
                    || (self.walls[i+1][j] && self.walls[i][j-1] && self.walls[i-1][j+1])
                    || (self.walls[i+1][j] && self.walls[i][j+1] && self.walls[i-1][j-1]);

                if corridor && !is_corridor {
                    continue;
                }

                let mut neighboors = vec![];
                if !self.walls[i + 1][j] {
                    neighboors.push((i + 1, j));
                }
                if !self.walls[i - 1][j] {
                    neighboors.push((i - 1, j));
                }
                if !self.walls[i][j - 1] {
                    neighboors.push((i, j - 1));
                }
                if !self.walls[i][j + 1] {
                    neighboors.push((i, j + 1));
                }

                for n in neighboors {
                    if unvisited.contains(&n) {
                        to_visit.insert(n);
                    }
                }

                unvisited.remove(&cell);
                zone.push(cell)
            }
            zones.push(zone);
        }

        zones
    }

    /// Compute the largest zone and fill all other zone
    pub fn fill_smallest(&mut self) {
        let mut zones = self.compute_zones(false);
        if zones.is_empty() {
            return;
        }
        let (_, max_id) = zones.iter().enumerate().fold(
            (-1, None),
            |(max_len, max_id), (id, zone)| {
                let len = zone.len() as isize;
                if len >= max_len {
                    (len, Some(id))
                } else {
                    (max_len, max_id)
                }
            },
        );
        zones.remove(max_id.unwrap());
        zones.iter().flat_map(|zone| zone.iter()).for_each(
            |&(i, j)| {
                self.walls[i][j] = true
            },
        );
    }

    pub fn fill_dead_end(&mut self) {
        loop {
            let mut corridors = self.compute_zones(true);
            corridors.retain(|corridor| {
                corridor.iter().any(|&(i, j)| {
                    (self.walls[i-1][j] && self.walls[i][j-1] && self.walls[i][j+1])
                    || (self.walls[i+1][j] && self.walls[i][j-1] && self.walls[i][j+1])
                    || (self.walls[i][j-1] && self.walls[i-1][j] && self.walls[i+1][j])
                    || (self.walls[i][j+1] && self.walls[i-1][j] && self.walls[i+1][j])
                })
            });
            if corridors.len() == 0 {
                break;
            }
            for &(i, j) in corridors.iter().flat_map(|z| z) {
                self.walls[i][j] = true;
            }
        }
    }

    pub fn random_free(&self) -> (usize, usize) {
        let x_range = Range::new(0, self.width);
        let y_range = Range::new(0, self.height);
        let mut rng = ::rand::thread_rng();

        let mut x = x_range.ind_sample(&mut rng);
        let mut y = x_range.ind_sample(&mut rng);
        while self.walls[x][y] {
            x = x_range.ind_sample(&mut rng);
            y = x_range.ind_sample(&mut rng);
        }
        (x, y)
    }

    pub fn random_free_float(&self) -> [f32; 2] {
        let cell = self.random_free();
        [cell.0 as f32 + 0.5, cell.1 as f32 + 0.5]
    }

    pub fn find_path(
        &self,
        pos: (usize, usize),
        goal: (usize, usize),
    ) -> Option<(Vec<(usize, usize)>, usize)> {
        ::pathfinding::astar(
            &pos,
            |&(x, y)| {
                let right = !self.walls[x + 1][y];
                let left = x > 0 && !self.walls[x - 1][y];
                let up = !self.walls[x][y + 1];
                let down = y > 0 && !self.walls[x][y - 1];
                let down_left = x > 0 && y > 0 && !self.walls[x - 1][y - 1];
                let down_right = y > 0 && !self.walls[x + 1][y - 1];
                let up_left = x > 0 && !self.walls[x - 1][y + 1];
                let up_right = !self.walls[x + 1][y + 1];

                let mut res = vec![];

                if right {
                    res.push(((x + 1, y), 10))
                }
                if left {
                    res.push(((x - 1, y), 10))
                }
                if up {
                    res.push(((x, y + 1), 10))
                }
                if down {
                    res.push(((x, y - 1), 10))
                }
                if up && right && up_right {
                    res.push(((x + 1, y + 1), 15))
                }
                if up && left && up_left {
                    res.push(((x - 1, y + 1), 15))
                }
                if down && right && down_right {
                    res.push(((x + 1, y - 1), 15))
                }
                if down && left && down_left {
                    res.push(((x - 1, y - 1), 15))
                }

                res
            },
            |&(x, y)| {
                let dx = if x > goal.0 { x - goal.0 } else { goal.0 - x };
                let dy = if y > goal.1 { y - goal.1 } else { goal.1 - y };
                ::std::cmp::min(dx, dy) * 10
            },
            |&p| p == goal,
        )
    }

    pub fn free_in_circle(&self, center: [usize; 2], radius: usize) -> Vec<[usize; 2]> {
        unimplemented!();
    }

    pub fn free_in_square(&self, center: [usize; 2], radius: usize) -> Vec<[usize; 2]> {
        let mut res = vec![];

        let top = center[1] as isize + radius as isize;
        let left = center[0] as isize - radius as isize;
        let right = center[0] as isize + radius as isize;
        let bottom = center[1] as isize - radius as isize;

        for &j in &[top, bottom] {
            if j >= 0 && j < self.height as isize {
                for i in left.max(0).min(self.width as isize)..(right+1).max(0).min(self.width as isize) {
                    if !self.walls[i as usize][j as usize] {
                        res.push([i as usize, j as usize]);
                    }
                }
            }
        }

        for &i in &[left, right] {
            if i >= 0 && i < self.width as isize {
                for j in bottom.max(0).min(self.height as isize)..(top+1).max(0).min(self.height as isize) {
                    if !self.walls[i as usize][j as usize] {
                        res.push([i as usize, j as usize]);
                    }
                }
            }
        }

        res
    }
}

/// Generate partial reverse randomized_kruskal
/// `https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_Kruskal.27s_algorithm`
pub fn kruskal(width: usize, height: usize, percent: f64) -> Maze {
    enum WallPos {
        Vertical(usize, usize),
        Horizontal(usize, usize),
    }

    assert_eq!(width.wrapping_rem(2), 1);
    assert_eq!(height.wrapping_rem(2), 1);

    let index = |x: usize, y: usize| y * width + x;

    let mut grid = Vec::with_capacity(width * height);
    for i in 0..width * height {
        grid.push((false, i));
    }

    for i in 0..width {
        grid[i] = (true, i);
        let j = height * (width - 1) + i;
        grid[j] = (true, j);
    }

    for i in 0..height {
        grid[i * width] = (true, i * width);
        let j = (i + 1) * width - 1;
        grid[j] = (true, j);
    }

    let horizontal_wall = (width - 5) / 2 * (height - 3) / 2;
    let vertical_wall = (width - 3) / 2 * (height - 5) / 2;
    let horizontal_wall_width = (width - 5) / 2;
    let vertical_wall_width = (width - 3) / 2;

    let mut walls = Vec::with_capacity(horizontal_wall + vertical_wall);
    for i in 0..vertical_wall {
        walls.push(WallPos::Vertical(
            i.wrapping_rem(vertical_wall_width) * 2 + 2,
            (i / vertical_wall_width) * 2 + 3,
        ));
    }
    for i in 0..horizontal_wall {
        walls.push(WallPos::Horizontal(
            i.wrapping_rem(horizontal_wall_width) * 2 + 3,
            (i / horizontal_wall_width) * 2 + 2,
        ));
    }

    let mut rng = ::rand::thread_rng();

    let stop = ((walls.len() as f64) * (1. - percent / 100.)) as usize;

    while walls.len() > stop {
        let i = ::rand::distributions::Range::new(0, walls.len()).ind_sample(&mut rng);
        assert!(i < walls.len());
        let (c1, c2, c3) = match walls.swap_remove(i) {
            WallPos::Vertical(x, y) => (index(x, y - 1), index(x, y), index(x, y + 1)),
            WallPos::Horizontal(x, y) => (index(x - 1, y), index(x, y), index(x + 1, y)),
        };

        let ((_, s1), (_, s2), (_, s3)) = (grid[c1], grid[c2], grid[c3]);

        if s1 != s3 {
            grid[c1] = (true, s1);
            grid[c2] = (true, s2);
            grid[c3] = (true, s3);
            for &mut (_, ref mut s) in &mut grid {
                if *s == s2 || *s == s3 {
                    *s = s1;
                }
            }
        }
    }

    let mut res = Vec::with_capacity(width);
    for i in 0..width {
        res.push(Vec::with_capacity(height));
        for j in 0..height {
            res[i].push(grid[index(i, j)].0);
        }
    }

    Maze {
        width,
        height,
        walls: res,
    }
}
