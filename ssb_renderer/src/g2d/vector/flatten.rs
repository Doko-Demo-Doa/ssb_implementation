// Imports
use super::{
    types::{Coordinate,Degree},
    point::{Point,GenericPoint}
};


// Flatten arc to polyline
const ARC_LINE_LENGTH: Degree = 0.75;
pub fn flatten_arc(start_point: Point, center_point: Point, angle: Degree) -> Vec<Point> {
    // Anything to do?
    if start_point == center_point || angle == 0.0 {
        return vec![start_point];
    }
    // Number of required lines
    let (vector, mut angle_rad) = (start_point - center_point, angle.to_radians());
    let lines_n = (angle_rad.abs() * vector.len() as Degree / ARC_LINE_LENGTH).ceil() as usize;
    // Points buffer with start point
    let mut points = Vec::with_capacity(1 + lines_n);
    points.push(start_point);
    // Add remaining points
    angle_rad /= lines_n as Degree;
    let (angle_sin, angle_cos) = (angle_rad.sin(), angle_rad.cos());
    let mut precise_vector = GenericPoint {
        x: vector.x as Degree,
        y: vector.y as Degree
    };
    for _ in 0..lines_n {
        points.push(center_point + {
            precise_vector = GenericPoint {
                x: precise_vector.x * angle_cos - precise_vector.y * angle_sin,
                y: precise_vector.x * angle_sin + precise_vector.y * angle_cos
            };
            Point {
                x: precise_vector.x as Coordinate,
                y: precise_vector.y as Coordinate
            }
        });
    }
    // Return points
    points
}

// Flatten curve to polyline
const CURVE_DEVIATION_LENGTH: Coordinate = 0.25;
#[inline]
fn is_curve_flat(p: &[Point;4]) -> bool {
    (p[1] - p[0]).grid_len() + (p[2] - p[1]).grid_len() + (p[3] - p[2]).grid_len()
    <
    (p[3] - p[0]).grid_len() + CURVE_DEVIATION_LENGTH
}
#[inline]
fn split_curve_mid(p: [Point;4]) -> ([Point;4], [Point;4]) {
    // Calculate intermediate points
    const T: Coordinate = 0.5;
    let (p01, p12, p23) = (
        p[0] + (p[1] - p[0]) * T,
        p[1] + (p[2] - p[1]) * T,
        p[2] + (p[3] - p[2]) * T
    );
    let (p012, p123) = (
        p01 + (p12 - p01) * T,
        p12 + (p23 - p12) * T
    );
    let p1234 = p012 + (p123 - p012) * T;
    // Assemble both curves
    (
        [p[0], p01, p012, p1234],
        [p1234, p123, p23, p[3]]
    )
}
pub fn flatten_curve(start_point: Point, control_point1: Point, control_point2: Point, end_point: Point) -> Vec<Point> {
    let mut points = vec![start_point];
    let mut curves = vec![[start_point, control_point1, control_point2, end_point]];
    while let Some(curve) = curves.pop() {
        // Flat enough = line
        if is_curve_flat(&curve) {
            points.push(curve[3]);
        } else {
            // Try again with subdivided curve
            let (curve1, curve2) = split_curve_mid(curve);
            curves.push(curve2);
            curves.push(curve1);
        }
    }
    points
}


// Tests
#[cfg(test)]
mod tests {
    use super::{Point, flatten_curve, flatten_arc};

    #[test]
    fn flat_arc() {
        // Nothing to do
        assert_eq!(
            flatten_arc(Point {x: 42.5, y: 33.0}, Point {x: 42.5, y: 33.0}, 99.0),
            vec![Point {x: 42.5, y: 33.0}]
        );
        assert_eq!(
            flatten_arc(Point {x: 1.0, y: 2.0}, Point {x: 3.0, y: 4.0}, 0.0),
            vec![Point {x: 1.0, y: 2.0}]
        );
        // Tiny angle = 2 points = 1 line
        assert_eq!(
            flatten_arc(Point {x: 0.0, y: -5.0}, Point {x: 0.0, y: 0.0}, 0.00001).len(),
            2
        );
        // Complex
        let points = flatten_arc(Point {x: 0.0, y: -4.0}, Point {x: 0.0, y: -2.0}, -270.0);
        assert_eq!(points.last(), Some(&Point {x: 2.0, y: -2.0}), "Last point not fitting: {:?}", points);
    }

    #[test]
    fn flat_curve() {
        // Already flat/line
        assert_eq!(
            flatten_curve(Point {x: -2.0, y: 7.0}, Point {x: -1.0, y: 7.0}, Point {x: 0.0, y: 7.0}, Point {x: 1.0, y: 7.0}),
            vec![Point {x: -2.0, y: 7.0}, Point {x: 1.0, y: 7.0}]
        );
        // Complex
        let points = flatten_curve(Point {x: -5.0, y: 0.0}, Point {x: 0.0, y: -4.0}, Point {x: 5.0, y: 6.0}, Point {x: 10.0, y: 1.0});
        assert!(points.len() > 5, "Not enough points: {:?}", points);
    }
}