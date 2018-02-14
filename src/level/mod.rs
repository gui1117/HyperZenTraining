pub mod kill_all_kruskal;
use rand::distributions::{IndependentSample, Range};
use std::ops::Mul;
use std::hash::Hash;
use typenum;

mod hall;
pub use self::hall::create_hall;

#[derive(Serialize, Deserialize, Clone)]
pub enum Level {
    KillAllKruskal2D(kill_all_kruskal::Conf2D),
    KillAllKruskal3D(kill_all_kruskal::Conf3D),
}

impl Level {
    pub fn create(&self, world: &mut ::specs::World) {
        match *self {
            Level::KillAllKruskal2D(ref conf) => conf.create(world),
            Level::KillAllKruskal3D(ref conf) => conf.create(world),
        }
    }
}

pub struct KruskalDecorated<D>
where
    D: ::na::Dim + ::na::DimName + Hash,
    D::Value: Mul<typenum::UInt<typenum::UTerm, typenum::B1>, Output = D::Value>
        + ::generic_array::ArrayLength<isize> + ::generic_array::ArrayLength<f32>,
{
    maze: ::maze::Maze<D>,
    start_cell: ::na::VectorN<isize, D>,
    start_opening: ::na::VectorN<isize, D>,
    end_cell: ::na::VectorN<isize, D>,
    end_opening: ::na::VectorN<isize, D>,
    entity_cells: Vec<::na::VectorN<isize, D>>,
    turret_cells: Vec<::na::VectorN<isize, D>>,
}

impl<D> KruskalDecorated<D>
where
    D: ::na::Dim + ::na::DimName + Hash,
    D::Value: Mul<typenum::UInt<typenum::UTerm, typenum::B1>, Output = D::Value>
        + ::generic_array::ArrayLength<isize> + ::generic_array::ArrayLength<f32>,
{
    /// we choose start room.
    /// then end room the further from start
    /// in rooms cells we put turret exept in front of end and start room
    /// in all cells exept turret and start room we put entities
    /// and all other things
    pub fn new(size: ::na::VectorN<isize, D>, percent: f64, bug: ::na::VectorN<isize, D>, turrets: usize, entities: usize) -> Self {
        let mut rng = ::rand::thread_rng();
        loop {
            // Generate general maze
            let mut maze = ::maze::Maze::kruskal(size.clone(), percent, bug.clone(), 1.0);
            maze.reduce(1);
            maze.circle();
            maze.fill_smallests();

            while maze.fill_dead_corridors() {}

            maze.extend(1);
            maze.circle();

            // Start
            let mut dig_start = maze.dig_cells(1, |_| true);
            if dig_start.first().is_none() { continue }
            let (start_cell, start_opening) = dig_start.remove(0);

            // End
            let mut dig_end = maze.dig_cells(1, |_| true);
            if dig_end.first().is_none() { continue }
            let (end_cell, end_opening) = dig_end.remove(0);

            // Put turrets
            let cells = maze.compute_inner_room_zones()
                .iter()
                .cloned()
                .filter_map(|mut room| {
                    room.retain(|cell| {
                        (start_cell.clone() - cell.clone()).iter().fold(0, |acc, c| acc + c.pow(2)) > 5_isize.pow(2)
                        && *cell != start_cell
                        && *cell != start_opening
                        && *cell != end_cell
                        && *cell != end_opening
                        && maze.is_neighbouring_wall(cell)
                    });
                    if room.len() == 0 {
                        None
                    } else {
                        let cell = room.iter()
                            .skip(Range::new(0, room.len()).ind_sample(&mut rng))
                            .next()
                            .unwrap()
                            .clone();
                        Some(cell)
                    }
                })
                .collect::<Vec<_>>();

            let mut turret_cells = vec![];

            for (_, cell) in (0..turrets).zip(cells) {
                turret_cells.push(cell);
            }

            // Put entities
            let mut cells = maze.iterate_maze();
            cells.retain(|cell| {
                (start_cell.clone() - cell.clone()).iter().fold(0, |acc, c| acc + c.pow(2)) > 5_isize.pow(2)
                && *cell != start_cell
                && *cell != start_opening
                && *cell != end_cell
                && *cell != end_opening
                && !turret_cells.contains(cell)
            });

            let mut entity_cells = vec![];

            for _ in 0..entities {
                let index = Range::new(0, cells.len()).ind_sample(&mut rng);
                let cell = cells.swap_remove(index);
                entity_cells.push(cell);
            }

            break KruskalDecorated {
                maze,
                start_cell,
                start_opening,
                end_cell,
                end_opening,
                entity_cells,
                turret_cells,
            }
        }
    }
}
