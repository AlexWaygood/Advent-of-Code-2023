use std::fs::read_to_string;
use std::str::FromStr;

use anyhow::{bail, Result};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    #[cfg(test)]
    fn origin() -> Self {
        Self::new(0, 0)
    }

    #[cfg(test)]
    fn rounded(&self) -> (u64, u64) {
        (self.x.round() as u64, self.y.round() as u64)
    }

    fn lies_within(&self, area: Area) -> bool {
        area.min <= self.x && self.x <= area.max && area.min <= self.y && self.y <= area.max
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let [x, y, _] = s
            .split(", ")
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?[..]
        else {
            bail!("Expected there to be exactly two commas in the position-list")
        };
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Copy)]
struct Area {
    min: f64,
    max: f64,
}

impl Area {
    const fn new(min: f64, max: f64) -> Self {
        Area { min, max }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    dy: i64,
    dx: i64,
}

impl Vector {
    fn resolve_gradient(&self) -> f64 {
        (self.dy as f64) / (self.dx as f64)
    }
}

impl FromStr for Vector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let [dx, dy, _] = s
            .split(", ")
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?[..]
        else {
            bail!("Expected there to be exactly two commas in the position-list")
        };
        Ok(Self { dx, dy })
    }
}

#[derive(Debug, PartialEq)]
enum LineRelationship {
    Equal,
    ParallelButNonEqual,
    NonParallelButNonIntersecting,
    NonParallelAndIntersecting { intersection: Point },
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct HailstoneTrajectory {
    known_point: Point,
    vector: Vector,
}

impl HailstoneTrajectory {
    fn new(known_point: Point, gradient: Vector) -> Self {
        Self {
            known_point,
            vector: gradient,
        }
    }

    fn y_intercept(&self) -> f64 {
        y_intercept_from_position_and_gradient(self.known_point, self.vector)
    }

    fn relationship_to(&self, other: &HailstoneTrajectory) -> LineRelationship {
        let (this_intercept, other_intercept) = (self.y_intercept(), other.y_intercept());
        let this_gradient = self.vector.resolve_gradient();
        let other_gradient = other.vector.resolve_gradient();
        if this_gradient == other_gradient {
            if this_intercept == other_intercept {
                return LineRelationship::Equal;
            }
            return LineRelationship::ParallelButNonEqual;
        }
        let seconds =
            (other.known_point.x - self.known_point.x) / (self.vector.dx - other.vector.dx) as f64;
        if seconds < 0.0 {
            return LineRelationship::NonParallelButNonIntersecting;
        }
        let intersection_x = self.known_point.x + (self.vector.dx as f64 * seconds);
        let intersection_y = self.known_point.y + (self.vector.dy as f64 * seconds);
        if other.known_point.y + (other.vector.dy as f64 * seconds) != intersection_y {
            return LineRelationship::NonParallelButNonIntersecting;
        }
        LineRelationship::NonParallelAndIntersecting {
            intersection: Point::new(intersection_x, intersection_y),
        }
    }
}

fn y_intercept_from_position_and_gradient(pos: Point, gradient: Vector) -> f64 {
    if pos.x == 0.0 {
        return pos.y;
    }
    pos.y - (gradient.resolve_gradient() * pos.x)
}

impl FromStr for HailstoneTrajectory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let [pos_info, gradient_info] = s.split(" @ ").collect_vec()[..] else {
            bail!("Expected there to be exactly one ` @ ` in each row")
        };
        Ok(Self::new(
            Point::from_str(pos_info)?,
            Vector::from_str(gradient_info)?,
        ))
    }
}

fn parse_input(filename: &str) -> Result<Vec<HailstoneTrajectory>> {
    let input = read_to_string(filename)?;
    input.lines().map(|line| line.parse()).collect()
}

fn solve(hailstone_trajectories: Vec<HailstoneTrajectory>, area_to_search: Area) -> usize {
    hailstone_trajectories
        .iter()
        .combinations(2)
        .inspect(|comb| println!("{:?}\n{:?}", comb[0], comb[1]))
        .map(|comb| comb[0].relationship_to(comb[1]))
        .inspect(|rel| println!("{rel:?}\n"))
        .filter(|relationship| match relationship {
            LineRelationship::Equal => true,
            LineRelationship::ParallelButNonEqual
            | LineRelationship::NonParallelButNonIntersecting => false,
            LineRelationship::NonParallelAndIntersecting { intersection } => {
                intersection.lies_within(area_to_search)
            }
        })
        .count()
}

fn main() {
    let hailstone_trajectories = parse_input("input.txt").unwrap();
    debug_assert_eq!(hailstone_trajectories.len(), 300);
    let area_to_search = Area::new(200_000_000_000_000.0, 400_000_000_000_000.0);
    let solution = solve(hailstone_trajectories, area_to_search);
    println!("{solution}");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_point() {
        let origin = Point::origin();
        assert!(origin.lies_within(Area::new(-1.0, 1.0)));
        assert!(origin.lies_within(Area::new(0.0, 1.0)));
        assert!(origin.lies_within(Area::new(-1.0, 0.0)));
        assert!(!origin.lies_within(Area::new(1.0, 2.0)));
        assert!(!origin.lies_within(Area::new(-2.0, -1.0)));
    }

    #[test]
    fn test_point_from_str() -> Result<()> {
        let p = Point::from_str("144788461200241, 195443318499267, 285412990927879")?;
        assert_eq!(p.rounded(), (144788461200241, 195443318499267));
        let p2 = Point::from_str("266680201159206, 319693757705834, 207679493757440")?;
        assert_eq!(p2.rounded(), (266680201159206, 319693757705834));
        Ok(())
    }

    #[test]
    fn test_gradient_from_str() -> Result<()> {
        let g = Vector::from_str("227, 158, 5")?;
        assert_eq!(g, Vector { dx: 227, dy: 158 });
        let g2 = Vector::from_str("37, -56, 138")?;
        assert_eq!(g2, Vector { dx: 37, dy: -56 });
        Ok(())
    }

    #[test]
    fn test_hailstone_from_str() -> Result<()> {
        let h = HailstoneTrajectory::from_str("0.0, 1.0, 216398516914389 @ -22, -140, 7")?;
        let expected = HailstoneTrajectory::new(Point::new(0, 1), Vector { dy: -140, dx: -22 });
        assert_eq!(h, expected);
        Ok(())
    }

    #[test]
    fn test_y_intercept_from_position_and_gradient() {
        let origin = Point::origin();
        let point1 = Point::new(1, 0);
        let point2 = Point::new(0, 1);
        let point3 = Point::new(1, 1);

        let h = Vector { dy: 0, dx: 1 };
        assert_eq!(y_intercept_from_position_and_gradient(origin, h), 0.0);
        assert_eq!(y_intercept_from_position_and_gradient(point1, h), 0.0);
        assert_eq!(y_intercept_from_position_and_gradient(point2, h), 1.0);
        assert_eq!(y_intercept_from_position_and_gradient(point3, h), 1.0);

        let f = Vector { dy: 1, dx: 1 };
        assert_eq!(y_intercept_from_position_and_gradient(origin, f), 0.0);
        assert_eq!(y_intercept_from_position_and_gradient(point1, f), -1.0);
        assert_eq!(y_intercept_from_position_and_gradient(point2, f), 1.0);
        assert_eq!(y_intercept_from_position_and_gradient(point3, f), 0.0);

        let point4 = Point::new(19, 13);
        let h2 = Vector { dy: 1, dx: -2 };
        assert_eq!(y_intercept_from_position_and_gradient(point4, h2), 22.5);

        let point5 = Point::new(-1, -1);
        let h3 = Vector { dy: -1, dx: -1 };
        assert_eq!(y_intercept_from_position_and_gradient(point5, h3), 0.0);
    }

    #[test]
    fn test_hailstone_intersection() {
        let horizontal_g = Vector { dy: 0, dx: 1 };
        let fourtyfive_g = Vector { dy: 1, dx: 1 };

        let h = HailstoneTrajectory::new(Point::origin(), horizontal_g);
        let h1 = HailstoneTrajectory::new(Point::origin(), fourtyfive_g);
        let h2 = HailstoneTrajectory::new(Point::new(0, 1), horizontal_g);
        let h3 = HailstoneTrajectory::new(Point::new(0, 1), fourtyfive_g);

        assert_eq!(h.relationship_to(&h), LineRelationship::Equal);
        assert_eq!(h1.relationship_to(&h1), LineRelationship::Equal);
        assert_eq!(h2.relationship_to(&h2), LineRelationship::Equal);
        assert_eq!(h3.relationship_to(&h3), LineRelationship::Equal);

        assert_eq!(
            h.relationship_to(&h1),
            LineRelationship::NonParallelAndIntersecting {
                intersection: Point::origin()
            }
        );
        assert_eq!(
            h.relationship_to(&h2),
            LineRelationship::ParallelButNonEqual
        );
        assert_eq!(
            h.relationship_to(&h3),
            LineRelationship::NonParallelAndIntersecting {
                intersection: Point::new(-1, 0)
            }
        );
    }

    #[test]
    fn test_example() -> Result<()> {
        let example = "\
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";
        let hailstones = example
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<HailstoneTrajectory>>>()?;
        for hailstone in &hailstones {
            println!("{hailstone:?}");
        }
        println!();
        assert_eq!(hailstones.len(), 5);
        let area_to_search = Area::new(7.0, 27.0);
        assert_eq!(solve(hailstones, area_to_search), 2);
        Ok(())
    }
}
