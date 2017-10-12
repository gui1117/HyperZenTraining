use rand::distributions::IndependentSample;

#[derive(Clone)]
pub struct Maze {
    pub walls: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
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
