use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::f32::consts::{FRAC_PI_2, PI};
use std::time::{Duration, Instant};
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
}

impl Direction {
    #[inline]
    #[allow(dead_code)]
    fn orthogonal(self, other: Self) -> bool {
        use self::Direction::*;
        match (self, other) {
            (Forward, Forward)
            | (Forward, Backward)
            | (Backward, Forward)
            | (Backward, Backward) => false,
            _ => true,
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn perpendicular(self, other: Self) -> bool {
        !self.orthogonal(other)
    }
}

pub fn high_byte(b: u32) -> u32 {
    b >> 8 as u8 as u32
}

pub fn low_byte(b: u32) -> u32 {
    b as u8 as u32
}

pub trait Pop {
    type Item;
    fn pop(&mut self) -> Option<Self::Item>;
}

impl<T: Eq + Hash + Clone> Pop for HashSet<T> {
    type Item = T;
    fn pop(&mut self) -> Option<Self::Item> {
        self.iter()
            .next()
            .map(|cell| cell.clone())
            .map(|cell| self.take(&cell).unwrap())
    }
}

pub trait ConvCoord {
    fn axis_angle_z(&self) -> ::na::Vector3<f32>;
}

impl ConvCoord for ::na::Vector2<isize> {
    fn axis_angle_z(&self) -> ::na::Vector3<f32> {
        ::na::Vector3::new(self[0], self[1], 0).axis_angle_z()
    }
}

impl ConvCoord for ::na::Vector3<isize> {
    fn axis_angle_z(&self) -> ::na::Vector3<f32> {
        if *self == ::na::Vector3::new(-1, 0, 0) {
            ::na::Vector3::new(0.0, -FRAC_PI_2, 0.0)
        } else if *self == ::na::Vector3::new(1, 0, 0) {
            ::na::Vector3::new(0.0, FRAC_PI_2, 0.0)
        } else if *self == ::na::Vector3::new(0, -1, 0) {
            ::na::Vector3::new(FRAC_PI_2, 0.0, 0.0)
        } else if *self == ::na::Vector3::new(0, 1, 0) {
            ::na::Vector3::new(-FRAC_PI_2, 0.0, 0.0)
        } else if *self == ::na::Vector3::new(0, 0, -1) {
            ::na::Vector3::new(PI, 0.0, 0.0)
        } else if *self == ::na::Vector3::new(0, 0, 1) {
            ::na::Vector3::new(0.0, 0.0, 0.0)
        } else {
            panic!("invalid direction");
        }
    }
}

const BENCHMARKER_VECDEQUE_SIZE: usize = 60;

pub struct Benchmark {
    name: String,
    min: Duration,
    max: Duration,
    mean: Duration,
}

impl fmt::Display for Benchmark {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let min = self.min.as_secs() as f64 + self.min.subsec_nanos() as f64 * 1e-9;
        let max = self.max.as_secs() as f64 + self.max.subsec_nanos() as f64 * 1e-9;
        let mean = self.mean.as_secs() as f64 + self.mean.subsec_nanos() as f64 * 1e-9;
        write!(
            f,
            "benchmark: {}\n\tmin: {}\n\tmax: {}\n\tmean: {}",
            self.name, min, max, mean
        )
    }
}

pub struct Benchmarker {
    instant: HashMap<String, Instant>,
    durations: HashMap<String, VecDeque<Duration>>,
}

impl Benchmarker {
    pub fn new() -> Self {
        Benchmarker {
            instant: HashMap::new(),
            durations: HashMap::new(),
        }
    }

    pub fn start(&mut self, name: &'static str) {
        assert_eq!(
            self.instant.insert(String::from(name), Instant::now()),
            None
        );
    }

    pub fn end(&mut self, name: &'static str) {
        if let Some(instant) = self.instant.remove(&String::from(name)) {
            let vecdeque = self.durations
                .entry(String::from(name))
                .or_insert_with(|| VecDeque::new());
            vecdeque.push_front(instant.elapsed());
            vecdeque.truncate(BENCHMARKER_VECDEQUE_SIZE);
        }
    }

    pub fn get_all(&self) -> Vec<Benchmark> {
        let mut res = Vec::new();
        for (name, durations) in &self.durations {
            let mut min = Duration::new(1000u64, 0);
            let mut max = Duration::new(0, 0);
            let mut sum = Duration::new(0, 0);
            for duration in durations {
                sum += *duration;
                min = min.min(*duration);
                max = max.max(*duration);
            }

            res.push(Benchmark {
                name: name.clone(),
                min,
                max,
                mean: sum / BENCHMARKER_VECDEQUE_SIZE as u32,
            });
        }
        res
    }
}

macro_rules! try_multiple_time {
    ($e:expr, $n:expr, $s:expr) => (
        {
            let mut error_timer = 0;
            let mut res = $e;
            while res.is_err() {
                ::std::thread::sleep(::std::time::Duration::from_millis($s));
                error_timer += 1;
                if error_timer > $n {
                    break;
                }
                res = $e;
            }
            res
        }
    )
}
