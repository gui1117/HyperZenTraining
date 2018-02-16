use std::collections::HashMap;
use std::f32::consts::*;

pub const POINT_RADIUS: f32 = 0.1;
pub const POINT_CENTER_DISTANCE: f32 = (1.0-2.0*POINT_RADIUS)/4.0;
pub const ARC_DIVISION: usize = 16;
pub const DEFAULT_CHAR: char = '▒';

pub fn build_text(text: String) -> Vec<[f32; 3]> {
    let mut offset = 0;
    let mut v = vec![];
    let default_glyph = GLYPHS.get(&DEFAULT_CHAR).unwrap();
    for character in text.chars() {
        if character == ' ' {
            offset += 1;
        } else {
            let glyph = GLYPHS.get(&character).unwrap_or(default_glyph);
            v.extend(glyph.vertices.iter().map(|p| [p[0] + offset as f32 * POINT_CENTER_DISTANCE, p[1], p[2]]));
            offset += glyph.size;
        }
    }
    v
}

// FIXME: use it in static draw that uses font
pub fn _get_size(text: String) -> usize {
    let mut size = 0;
    let default_glyph = GLYPHS.get(&DEFAULT_CHAR).unwrap();
    for character in text.chars() {
        if character == ' ' {
            size += 1
        } else {
            size += GLYPHS.get(&character).unwrap_or(default_glyph).size;
        }
    }
    size
}

lazy_static! {
    pub static ref GLYPHS: HashMap<char, Glyph> = {
        let mut glyphs = HashMap::new();
        for (character, operations) in operations().iter() {
            assert!(!operations.is_empty());
            glyphs.insert(*character, Glyph {
                vertices: operations.iter().flat_map(|op| op.vertices()).collect(),
                size: operations.iter().map(|op| op.size()).max().unwrap() + 1,
            });
        }
        glyphs
    };
}

pub struct Glyph {
    vertices: Vec<[f32; 3]>,
    size: usize,
}

/// points:
/// 00--05--10--15--20
/// ------------------
/// 01--06--11--16--21
/// ------------------
/// 02--07--12--17--22
/// ------------------
/// 03--08--13--18--23
/// ------------------
/// 04--09--14--19--24
///
/// Arcs are quarter of circle
/// in trigonometric orientation
///
/// Segment are from left to right and top to bottom
enum Operation {
    Segment((usize, usize), (usize, usize)),
    Arc((usize, usize), (usize, usize)),
}

impl Operation {
    fn size(&self) -> usize {
        match *self {
            Operation::Segment(p1, p2) => p1.0.max(p2.0),
            Operation::Arc(p1, p2) => p1.0.max(p2.0),
        }
    }

    fn point_coordinates(p: usize) -> (usize, usize) {
        match p {
            0 => (0, 4),
            1 => (0, 3),
            2 => (0, 2),
            3 => (0, 1),
            4 => (0, 0),
            5 => (1, 4),
            6 => (1, 3),
            7 => (1, 2),
            8 => (1, 1),
            9 => (1, 0),
            10 => (2, 4),
            11 => (2, 3),
            12 => (2, 2),
            13 => (2, 1),
            14 => (2, 0),
            15 => (3, 4),
            16 => (3, 3),
            17 => (3, 2),
            18 => (3, 1),
            19 => (3, 0),
            20 => (4, 4),
            21 => (4, 3),
            22 => (4, 2),
            23 => (4, 1),
            24 => (4, 0),
            _ => panic!(),
        }
    }

    fn arc(p1: usize, p2: usize) -> Self {
        let p1 = Self::point_coordinates(p1);
        let p2 = Self::point_coordinates(p2);
        assert!((p1.0 as isize - p2.0 as isize).abs() == (p1.1 as isize - p2.1 as isize).abs());
        Operation::Arc(p1, p2)
    }

    fn segment(p1: usize, p2: usize) -> Self {
        let p1 = Self::point_coordinates(p1);
        let p2 = Self::point_coordinates(p2);
        assert!(p1.0 < p2.0 || (p1.0 == p2.0 && p1.1 > p2.1));
        Operation::Segment(p1, p2)
    }

    fn vertices(&self) -> Vec<[f32; 3]> {
        match *self {
            Operation::Segment(p1, p2) => if p1.1 > p2.1 {
                let mut v = vec![];
                v.append(&mut letter_up_triangle([
                    top_right(p1),
                    top_left(p1),
                    bottom_left(p1),
                ]));
                v.append(&mut letter_up_triangle([
                    bottom_left(p2),
                    bottom_right(p2),
                    top_right(p2),
                ]));
                v.append(&mut letter_up_quad([
                    top_right(p1),
                    bottom_left(p1),
                    bottom_left(p2),
                    top_right(p2),
                ]));
                v
            } else {
                let mut v = vec![];
                v.append(&mut letter_up_triangle([
                    top_left(p1),
                    bottom_left(p1),
                    bottom_right(p1),
                ]));
                v.append(&mut letter_up_triangle([
                    bottom_right(p2),
                    top_right(p2),
                    top_left(p2),
                ]));
                v.append(&mut letter_up_quad([
                    top_left(p1),
                    bottom_right(p1),
                    bottom_right(p2),
                    top_left(p2),
                ]));
                v
            },
            Operation::Arc(p1, p2) => {
                letter_arc(p1, p2)
            }
        }
    }
}

fn center(p: (usize, usize)) -> ::na::Vector2<f32> {
    (::na::Vector2::new(p.0 as f32, p.1 as f32) * POINT_CENTER_DISTANCE) + ::na::Vector2::identity() * POINT_RADIUS
}

fn top_left(p: (usize, usize)) -> ::na::Vector2<f32> {
    center(p) + ::na::Vector2::new(-POINT_RADIUS, POINT_RADIUS)
}

fn top_right(p: (usize, usize)) -> ::na::Vector2<f32> {
    center(p) + ::na::Vector2::new(POINT_RADIUS, POINT_RADIUS)
}

fn bottom_left(p: (usize, usize)) -> ::na::Vector2<f32> {
    center(p) + ::na::Vector2::new(-POINT_RADIUS, -POINT_RADIUS)
}

fn bottom_right(p: (usize, usize)) -> ::na::Vector2<f32> {
    center(p) + ::na::Vector2::new(POINT_RADIUS, -POINT_RADIUS)
}

fn letter_arc(p1: (usize, usize), p2: (usize, usize)) -> Vec<[f32; 3]> {
    enum Quarter {
        TopRight,
        TopLeft,
        BottomLeft,
        BottomRight,
    }

    let quarter = if p1.0 > p2.0 && p1.1 < p2.1 {
        Quarter::TopRight
    } else if p1.0 > p2.0 && p1.1 > p2.1 {
        Quarter::TopLeft
    } else if p1.0 < p2.0 && p1.1 > p2.1 {
        Quarter::BottomLeft
    } else if p1.0 < p2.0 && p1.1 < p2.1 {
        Quarter::BottomRight
    } else {
        unreachable!();
    };

    let (start_angle, center) = match quarter {
        Quarter::TopRight => (0.0, top_right((p2.0, p1.1))),
        Quarter::TopLeft => (FRAC_PI_2, top_left((p1.0, p2.1))),
        Quarter::BottomLeft => (PI, bottom_left((p2.0, p1.1))),
        Quarter::BottomRight => (3.0*FRAC_PI_2, bottom_right((p1.0, p2.1))),
    };

    let mut inner_arc = vec![];
    let mut outer_arc = vec![];
    let inner_radius = (p1.0 as isize - p2.0 as isize).abs() as f32 * POINT_CENTER_DISTANCE - 2.0*POINT_RADIUS;
    let outer_radius = (p1.0 as isize - p2.0 as isize).abs() as f32 * POINT_CENTER_DISTANCE;

    for i in 0..ARC_DIVISION+1 {
        let angle = start_angle + FRAC_PI_2 * i as f32 / ARC_DIVISION as f32;
        let point = ::na::Vector2::new(angle.cos(), angle.sin());
        inner_arc.push(center + point*inner_radius);
        outer_arc.push(center + point*outer_radius);
    }

    let mut v = vec![];

    // Add letter up
    for i in 0..ARC_DIVISION {
        v.append(&mut letter_up_quad([
            inner_arc[i],
            outer_arc[i],
            outer_arc[i+1],
            inner_arc[i+1],
        ]));
    }
    v.append(&mut letter_up_point(p1));
    v.append(&mut letter_up_point(p2));
    v
}

fn letter_up_point(point: (usize, usize)) -> Vec<[f32; 3]> {
    letter_up_quad([
        top_right(point),
        top_left(point),
        bottom_left(point),
        bottom_right(point),
    ])
}

/// Points must be in trigonometric orientation for backface culling
fn letter_up_quad(points: [::na::Vector2<f32>; 4]) -> Vec<[f32; 3]> {
    [
        points[0],
        points[1],
        points[2],

        points[2],
        points[3],
        points[0],
    ].iter().map(|p| [p[0], p[1], 0.1]).collect()
}

/// Points must be in trigonometric orientation for backface culling
fn letter_up_triangle(points: [::na::Vector2<f32>; 3]) -> Vec<[f32; 3]> {
    points.iter().map(|p| [p[0], p[1], 0.1]).collect()
}

fn operations() -> HashMap<char, Vec<Operation>> {
    let mut map = HashMap::new();
    map.insert('A', vec![
               Operation::arc(11, 5),
               Operation::arc(5, 1),
               Operation::segment(1, 4),
               Operation::segment(11, 14),
               Operation::segment(2, 12),
    ]);
    map.insert('B', vec![
               Operation::segment(0, 4),
               Operation::segment(0, 5),
               Operation::segment(2, 7),
               Operation::segment(4, 9),
               Operation::arc(7, 11),
               Operation::arc(11, 5),
               Operation::arc(13, 7),
               Operation::arc(9, 13),
    ]);
    map.insert('C', vec![
               Operation::arc(10, 2),
               Operation::arc(2, 14),
    ]);
    map.insert('D', vec![
               Operation::arc(4, 12),
               Operation::arc(12, 0),
               Operation::segment(0, 4),
    ]);
    map.insert('E', vec![
               Operation::segment(0, 4),
               Operation::segment(0, 10),
               Operation::segment(2, 12),
               Operation::segment(4, 14),
    ]);
    map.insert('F', vec![
               Operation::segment(0, 4),
               Operation::segment(0, 10),
               Operation::segment(2, 12),
    ]);
    map.insert('G', vec![
               Operation::arc(10, 2),
               Operation::arc(2, 14),
               Operation::arc(14, 22),
               Operation::segment(17, 22),
    ]);
    map.insert('H', vec![
               Operation::segment(0, 4),
               Operation::segment(10, 14),
               Operation::segment(2, 12),
    ]);
    map.insert('I', vec![
               Operation::segment(0, 4),
    ]);
    map.insert('J', vec![
               Operation::segment(0, 10),
               Operation::segment(10, 12),
               Operation::arc(4, 12),
    ]);
    map.insert('K', vec![
               Operation::segment(0, 4),
               Operation::segment(2, 10),
               Operation::segment(2, 14),
    ]);
    map.insert('L', vec![
               Operation::segment(0, 4),
               Operation::segment(4, 14),
    ]);
    map.insert('M', vec![
               Operation::segment(0, 4),
               Operation::segment(0, 12),
               Operation::segment(12, 20),
               Operation::segment(20, 24),
    ]);
    map.insert('N', vec![
               Operation::segment(0, 4),
               Operation::segment(10, 14),
               Operation::segment(0, 14),
    ]);
    map.insert('O', vec![
               Operation::arc(10, 2),
               Operation::arc(2, 14),
               Operation::arc(14, 22),
               Operation::arc(22, 10),
    ]);
    map.insert('P', vec![
               Operation::segment(0, 4),
               Operation::segment(0, 5),
               Operation::segment(2, 7),
               Operation::arc(7, 11),
               Operation::arc(11, 5),
    ]);
    map.insert('Q', vec![
               Operation::arc(10, 2),
               Operation::arc(2, 14),
               Operation::arc(14, 22),
               Operation::arc(22, 10),
               Operation::segment(18, 24),
    ]);
    map.insert('R', vec![
               Operation::segment(0, 4),
               Operation::segment(0, 5),
               Operation::segment(2, 7),
               Operation::arc(7, 11),
               Operation::arc(11, 5),
               Operation::segment(2, 14),
    ]);
    map.insert('S', vec![
               Operation::segment(5, 10),
               Operation::segment(4, 9),
               Operation::arc(5, 1),
               Operation::arc(1, 7),
               Operation::arc(13, 7),
               Operation::arc(9, 13),
    ]);
    map.insert('T', vec![
               Operation::segment(5, 9),
               Operation::segment(0, 10),
    ]);
    map.insert('U', vec![
               Operation::segment(0, 3),
               Operation::segment(10, 13),
               Operation::arc(3, 9),
               Operation::arc(9, 13),
    ]);
    map.insert('V', vec![
               Operation::segment(0, 9),
               Operation::segment(9, 10),
    ]);
    map.insert('W', vec![
               Operation::segment(0, 9),
               Operation::segment(9, 13),
               Operation::segment(13, 19),
               Operation::segment(19, 20),
    ]);
    map.insert('X', vec![
               Operation::segment(0, 14),
               Operation::segment(4, 10),
    ]);
    map.insert('Y', vec![
               Operation::segment(0, 7),
               Operation::segment(7, 10),
               Operation::segment(7, 9),
    ]);
    map.insert('Z', vec![
               Operation::segment(0, 10),
               Operation::segment(4, 10),
               Operation::segment(4, 14),
    ]);
    map.insert('0', vec![
               Operation::arc(5, 1),
               Operation::arc(3, 9),
               Operation::arc(9, 13),
               Operation::arc(11, 5),
               Operation::segment(1, 3),
               Operation::segment(11, 13),
    ]);
    map.insert('1', vec![
               Operation::segment(2, 10),
               Operation::segment(10, 14),
               Operation::segment(9, 14),
    ]);
    map.insert('2', vec![
               Operation::segment(0, 5),
               Operation::arc(11, 5),
               Operation::segment(11, 12),
               Operation::segment(4, 12),
               Operation::segment(4, 14),
    ]);
    // map.insert('2', vec![
    //            Operation::segment(4, 14),
    //            Operation::arc(12, 0),
    //            Operation::arc(4, 12),
    // ]);
    map.insert('3', vec![
               Operation::segment(0, 5),
               Operation::segment(4, 9),
               Operation::arc(9, 13),
               Operation::arc(13, 7),
               Operation::arc(7, 11),
               Operation::arc(11, 5),
    ]);
    map.insert('4', vec![
               Operation::segment(3, 10),
               Operation::segment(3, 13),
               Operation::segment(12, 14),
    ]);
    map.insert('5', vec![
               Operation::segment(0, 10),
               Operation::segment(0, 2),
               Operation::segment(2, 7),
               Operation::segment(4, 9),
               Operation::arc(9, 13),
               Operation::arc(13, 7),
    ]);
    map.insert('6', vec![
               Operation::segment(2, 3),
               Operation::arc(10, 2),
               Operation::arc(7, 3),
               Operation::arc(3, 9),
               Operation::arc(9, 13),
               Operation::arc(13, 7),
    ]);
    map.insert('7', vec![
               Operation::segment(4, 10),
               Operation::segment(0, 10),
    ]);
    map.insert('8', vec![
               Operation::arc(5, 1),
               Operation::arc(1, 7),
               Operation::arc(7, 11),
               Operation::arc(11, 5),
               Operation::arc(7, 3),
               Operation::arc(3, 9),
               Operation::arc(9, 13),
               Operation::arc(13, 7),
    ]);
    map.insert('9', vec![
               Operation::arc(7, 11),
               Operation::arc(11, 5),
               Operation::arc(5, 1),
               Operation::arc(1, 7),
               Operation::segment(11, 12),
               Operation::arc(4, 12),
    ]);
    map.insert(':', vec![
               Operation::segment(2, 7),
               Operation::segment(4, 9),
    ]);
    map.insert('.', vec![
               Operation::segment(4, 9),
    ]);
    map.insert('-', vec![
               Operation::segment(2, 7),
    ]);
    map.insert('_', vec![
               Operation::segment(4, 14),
    ]);
    map.insert('▒', vec![
               Operation::segment(0, 10),
               Operation::segment(1, 11),
               Operation::segment(2, 12),
               Operation::segment(3, 13),
               Operation::segment(4, 14),
               Operation::segment(0, 4),
               Operation::segment(5, 9),
               Operation::segment(10, 14),
    ]);
    map
}
