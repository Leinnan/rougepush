use rand::{distributions::WeightedIndex, prelude::*};
use std::collections::HashSet;

use crate::vectors::Vector2Int;

pub struct Room {
    pub a: Vector2Int,
    pub b: Vector2Int,
}
impl Room {
    pub fn new(a: Vector2Int, b: Vector2Int) -> Self {
        // sort the coordinates so `a` is always the left-bottom
        // and `b` is top-right
        Room {
            a: Vector2Int::new(a.x.min(b.x), a.y.min(b.y)),
            b: Vector2Int::new(a.x.max(b.x), a.y.max(b.y)),
        }
    }
    pub fn corners(&self) -> [Vector2Int; 4] {
        [
            Vector2Int::new(self.a.x, self.a.y),
            Vector2Int::new(self.b.x, self.a.y),
            Vector2Int::new(self.b.x, self.b.y),
            Vector2Int::new(self.a.x, self.b.y),
        ]
    }
    pub fn random_point(&self) -> Vector2Int {
        let mut rng = thread_rng();
        let x = rng.gen_range(self.a.x..=self.b.x);
        let y = rng.gen_range(self.a.y..=self.b.y);
        Vector2Int::new(x, y)
    }
    pub fn random_point_without_walls(&self) -> Vector2Int {
        let mut rng = thread_rng();
        let x = rng.gen_range(self.a.x + 1..=self.b.x - 1);
        let y = rng.gen_range(self.a.y + 1..=self.b.y - 1);
        Vector2Int::new(x, y)
    }
    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        (self.a.y..=self.b.y)
            .flat_map(|y| (self.a.x..=self.b.x).map(move |x| Vector2Int::new(x, y)))
            .collect()
    }
}

pub struct Area {
    pub rooms: Vec<Room>,
    pub paths: Vec<Vec<Vector2Int>>,
    pub tunneler: Box<dyn Tunneler>,
}
impl Area {
    pub fn new(tunneler: Box<dyn Tunneler>) -> Self {
        Area {
            rooms: Vec::new(),
            paths: Vec::new(),
            tunneler,
        }
    }
    pub fn get_bounds(&self) -> (Vector2Int, Vector2Int) {
        let min_x = self.rooms.iter().map(|r| r.a.x).min().unwrap();
        let max_x = self.rooms.iter().map(|r| r.b.x).max().unwrap();
        let min_y = self.rooms.iter().map(|r| r.a.y).min().unwrap();
        let max_y = self.rooms.iter().map(|r| r.b.y).max().unwrap();
        (Vector2Int::new(min_x, min_y), Vector2Int::new(max_x, max_y))
    }
    pub fn get_size(&self) -> Vector2Int {
        let bounds = self.get_bounds();
        Vector2Int::new(bounds.1.x - bounds.0.x + 1, bounds.1.y - bounds.0.y + 1)
    }
    pub fn shift(&mut self, offset: Vector2Int) {
        // translate the entire area by a given offset
        let bounds = self.get_bounds();
        let d = offset - bounds.0;

        for room in self.rooms.iter_mut() {
            room.a += d;
            room.b += d;
        }
        for path in self.paths.iter_mut() {
            for v in path.iter_mut() {
                *v += d;
            }
        }
    }
    pub fn join_rooms(&self, a: &Room, b: &Room) -> Vec<Vector2Int> {
        self.tunneler.connect(a.random_point(), b.random_point())
    }
    pub fn generate_rooms(&mut self) {
        self.rooms = vec![
            Room::new(Vector2Int::new(0, 0), Vector2Int::new(4, 6)),
            Room::new(Vector2Int::new(10, 2), Vector2Int::new(14, 8)),
        ];
        self.paths = vec![self.join_rooms(&self.rooms[0], &self.rooms[1])];
    }
    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        self.rooms
            .iter()
            .flat_map(|r| r.to_tiles())
            .chain(self.paths.iter().flatten().copied())
            .collect()
    }

    fn find_closest_room_pair<'a>(&'a self, other: &'a Area) -> (&'a Room, &'a Room) {
        // find closest room pair between two areas
        // based on corner distances only
        let mut pairs = Vec::new();
        for ra in self.rooms.iter() {
            for rb in other.rooms.iter() {
                // find min corner dist
                let d = ra
                    .corners()
                    .iter()
                    .flat_map(|ca| {
                        rb.corners()
                            .iter()
                            .map(|cb| ca.manhattan(*cb))
                            .collect::<Vec<_>>()
                    })
                    .min()
                    .unwrap();
                pairs.push((d, ra, rb));
            }
        }
        // sort by corner dist
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        (pairs[0].1, pairs[0].2)
    }
    pub fn join_area(&self, other: &Area) -> Vec<Vector2Int> {
        let rooms = self.find_closest_room_pair(other);
        self.join_rooms(rooms.0, rooms.1)
    }
}
const AREA_SPACING: i32 = 4;

pub struct Dungeon {
    pub areas: Vec<Area>,
    // the gird contains indexes to the areas vec
    // rows / columns
    grid: Vec<Vec<usize>>,
}
impl Dungeon {
    pub fn new(row_count: usize) -> Self {
        let grid = (0..row_count).map(|_| Vec::new()).collect::<Vec<_>>();
        Dungeon {
            areas: Vec::new(),
            grid,
        }
    }
    pub fn add_area(&mut self, area: Area) {
        self.areas.push(area);
        let idx = self.areas.len() - 1;
        let row_count = self.grid.len();
        // insert the index to appropriate row vec
        self.grid[idx % row_count].push(idx);
    }
    pub fn generate(&mut self) {
        for area in self.areas.iter_mut() {
            area.generate_rooms();
        }
        self.position_areas();
        self.connect_areas();
    }
    fn connect_areas(&mut self) {
        // connect areas based on their grid location
        let mut pairs = Vec::new();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, idx) in row.iter().enumerate() {
                if x != 0 {
                    // join to area at x - 1
                    pairs.push((idx, row[x - 1]));
                };
                if y != 0 {
                    // join to area at y - 1
                    pairs.push((idx, self.grid[y - 1][x]));
                };
            }
        }
        for pair in pairs {
            let path = self.areas[*pair.0].join_area(&self.areas[pair.1]);
            self.areas[*pair.0].paths.push(path);
        }
    }
    pub fn to_tiles(&self) -> HashSet<Vector2Int> {
        self.areas.iter().flat_map(|a| a.to_tiles()).collect()
    }
    fn position_areas(&mut self) {
        let column_count = self.grid[0].len();

        // calculate grid dimensions based on contained areas
        let column_widths = (0..column_count)
            .map(|i| {
                self.grid
                    .iter()
                    .map(|r| match r.get(i) {
                        None => 0,
                        Some(_) => self.areas[i].get_size().x,
                    })
                    .max()
                    .unwrap()
                    + AREA_SPACING
            })
            .collect::<Vec<_>>();
        let row_heights = self
            .grid
            .iter()
            .map(|r| r.iter().map(|i| self.areas[*i].get_size().y).max().unwrap() + AREA_SPACING)
            .collect::<Vec<_>>();

        // calculate the offset amounts per each grid position
        let column_shifts = (0..column_widths.len())
            .map(|i| column_widths[..i].iter().sum())
            .collect::<Vec<i32>>();
        let row_shifts = (0..row_heights.len())
            .map(|i| row_heights[..i].iter().sum())
            .collect::<Vec<i32>>();

        // reposition the areas
        for (y, row) in self.grid.iter().enumerate() {
            for (x, idx) in row.iter().enumerate() {
                let offset = Vector2Int::new(column_shifts[x], row_shifts[y]);
                self.areas[*idx].shift(offset);
            }
        }
    }
}

pub trait Tunneler {
    fn connect(&self, a: Vector2Int, b: Vector2Int) -> Vec<Vector2Int>;
}

pub struct LShapeTunneler;
impl Tunneler for LShapeTunneler {
    // connects two points by forming an L shaped connection
    // initial direction (hor / ver) is the one whith the biggest coordinate difference
    fn connect(&self, a: Vector2Int, b: Vector2Int) -> Vec<Vector2Int> {
        let d = b - a;
        let (hor_y, ver_x) = match d.x > d.y {
            true => (a.y, b.x),
            false => (b.y, a.x),
        };

        let hor = (a.x.min(b.x)..=a.x.max(b.x))
            .map(|x| Vector2Int::new(x, hor_y))
            .collect::<Vec<_>>();
        let ver = (a.y.min(b.y)..=a.y.max(b.y))
            .map(|y| Vector2Int::new(ver_x, y))
            .collect::<Vec<_>>();
        [ver, hor].concat()
    }
}

pub struct RandomTunneler;
impl Tunneler for RandomTunneler {
    // connects two points by taking a random direction (hor / ver) towards the target
    // choice chance is determined by a current coordinate difference
    // (it is most likely to pick a dir with the biggest diff)
    fn connect(&self, a: Vector2Int, b: Vector2Int) -> Vec<Vector2Int> {
        let mut cur = a;
        let mut path = Vec::new();
        let mut rng = thread_rng();

        while cur != b {
            path.push(cur);
            // 0 is horizontal, 1 is vertical
            let dirs = [b.x - cur.x, b.y - cur.y];
            // build weights
            let dist = WeightedIndex::new(dirs.iter().map(|d| d.abs())).unwrap();
            // pick a dir idx (0 or 1)
            let dir_idx = dist.sample(&mut rng);
            // create a normalized step vector in a single direction
            let dv = match dir_idx {
                0 => Vector2Int::new(dirs[0] / dirs[0].abs(), 0),
                1 => Vector2Int::new(0, dirs[1] / dirs[1].abs()),
                _ => panic!(),
            };
            cur += dv;
        }
        path
    }
}
