use std::collections::{HashMap, HashSet};

type Segment = (i32, i32);
type BrickId = usize;

// (x, y)
type Location = (i32, i32);

fn sorted_segment(v1: i32, v2: i32) -> Segment {
    if v1 < v2 {
        (v1, v2)
    } else {
        (v2, v1)
    }
}

type Area = HashSet<Location>;
type IdArea = HashSet<(Location, BrickId)>;

#[derive(Debug, Clone)]
struct Brick {
    id: BrickId,
    area: Area,
    z: Segment,
}

type Grid = HashMap<i32, IdArea>;

const GROUND_Z: i32 = 0;

impl Brick {
    fn from_str(id: usize, s: &str) -> Self {
        let ends: Vec<Vec<i32>> = s
            .split("~")
            .map(|end| end.split(",").map(|n| n.parse().unwrap()).collect())
            .collect();

        let x = sorted_segment(ends[0][0], ends[1][0]);
        let y = sorted_segment(ends[0][1], ends[1][1]);
        Self {
            id,
            area: Self::area(x, y),
            z: sorted_segment(ends[0][2], ends[1][2]),
        }
    }

    fn area(x: Segment, y: Segment) -> HashSet<Location> {
        let mut area: HashSet<Location> = Default::default();
        let (start_x, end_x) = x;
        for x in start_x..=end_x {
            let (start_y, end_y) = y;
            for y in start_y..=end_y {
                area.insert((x, y));
            }
        }

        area
    }

    fn drop_brick(&mut self, grid: &mut Grid) -> HashSet<BrickId> {
        let (start_z, end_z) = self.z;
        let mut z = start_z;

        let mut supporting_bricks = HashSet::<BrickId>::new();
        while z > GROUND_Z + 1 {
            let z_below = z - 1;
            if let Some(current_floor_id_area) = grid.get(&z_below) {
                for (loc, id) in current_floor_id_area {
                    if self.area.contains(loc) {
                        supporting_bricks.insert(*id);
                    }
                }
            }

            if !supporting_bricks.is_empty() {
                break;
            }

            z = z_below;
        }

        // update to new z-span
        self.z = (z, z + end_z - start_z);

        // fill grid at all levels the brick occupies
        for fillable_z in self.z.0..=self.z.1 {
            grid.entry(fillable_z)
                .or_insert(Default::default())
                .extend(self.area.iter().map(|loc| (*loc, self.id)));
        }

        return supporting_bricks;
    }
}

fn remove(
    id: &BrickId,
    brick_to_supportees: &HashMap<BrickId, Vec<BrickId>>,
    brick_to_supporters: &HashMap<BrickId, Vec<BrickId>>,
    removed: &mut HashSet<BrickId>,
) {
    removed.insert(*id);
    if let Some(supportees) = brick_to_supportees.get(id) {
        for supportee in supportees {
            let supporters: Vec<_> = brick_to_supporters
                .get(supportee)
                .unwrap()
                .iter()
                .filter(|supporter| !removed.contains(supporter))
                .collect();
            if supporters.len() < 1 {
                remove(supportee, brick_to_supportees, brick_to_supporters, removed);
            }
        }
    }
}

fn main() {
    let mut bricks: HashMap<BrickId, Brick> = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .enumerate() // enumerating to have an id for each brick
        .map(|(id, l)| Brick::from_str(id, &l))
        .enumerate()
        .collect();

    let mut brick_ids_by_z: Vec<BrickId> = bricks.keys().into_iter().map(|id| *id).collect();
    brick_ids_by_z.sort_by_key(|id| bricks.get(id).unwrap().z);

    let mut brick_to_supportees = HashMap::<BrickId, Vec<BrickId>>::new();
    let mut brick_to_supporters = HashMap::<BrickId, Vec<BrickId>>::new();

    let mut grid: Grid = Default::default();
    for id in brick_ids_by_z.iter() {
        let brick = bricks.get_mut(&id).unwrap();
        let supporters = brick.drop_brick(&mut grid);

        for supporter in supporters {
            brick_to_supportees
                .entry(supporter)
                .or_insert(vec![])
                .push(*id);
            brick_to_supporters
                .entry(*id)
                .or_insert(vec![])
                .push(supporter);
        }
    }

    let sol: usize = brick_ids_by_z
        .iter()
        .map(|id| {
            let mut removed = HashSet::new();
            remove(id, &brick_to_supportees, &brick_to_supporters, &mut removed);
            // subtracting one because the first brick was added to removed in `remove`, but shouldn't be counted
            removed.len() - 1
        })
        .sum();
    println!("{}", sol);
}
