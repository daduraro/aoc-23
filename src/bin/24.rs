use std::str::FromStr;
use std::fmt::Debug;
use geo::{Line, Coord};
use geo::line_intersection::{line_intersection, LineIntersection};
use itertools::Itertools;
use nalgebra::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

advent_of_code::solution!(24);

fn parse<T>(input: &str) -> Vec<([T; 3], [T; 3])>
where 
    T: FromStr,
    <T as FromStr>::Err: Debug
{
    input.lines().map(|line|{
        let (p, v) = line.split_once(" @ ").unwrap();
        let mut p = p.split(',').map(|x| x.trim().parse::<T>().unwrap());
        let mut v = v.split(',').map(|x| x.trim().parse::<T>().unwrap());
        (
            [p.next().unwrap(), p.next().unwrap(), p.next().unwrap()],
            [v.next().unwrap(), v.next().unwrap(), v.next().unwrap()],
        )
    }).collect()
}

fn part_one_impl(input: &str, bounds: [f64; 2]) -> Option<u32> {
    let lines = parse::<f64>(input);

    let b = bounds[1] - bounds[0];

    let lines = lines.into_iter().map(|(p, v)|{
        Line::new(Coord{x: p[0], y: p[1]}, Coord{x: p[0] + b*v[0], y: p[1] + b*v[1]})
    }).collect_vec();

    let mut result = 0;
    for i in 0..lines.len() {
        for j in (i+1)..lines.len() {
            match line_intersection(lines[i], lines[j]) {
                Some(LineIntersection::SinglePoint { intersection, .. }) => {
                    if intersection.x >= bounds[0] && intersection.x <= bounds[1] && intersection.y >= bounds[0] && intersection.y <= bounds[1] {
                        result += 1;
                    }
                },
                Some(LineIntersection::Collinear { .. }) => {
                    result += 1;
                },
                _ => {},
            }
        }
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_impl(input, [200000000000000.0, 400000000000000.0])
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut s = parse::<f64>(input);
    s.shuffle(&mut thread_rng());

    // x0 + u0*t = x + u*t => t = (x-x0)/(u0-u) = (y-y0)/(v0-v) = (z-z0)/(w0-w)

    // consider only x,y,u,v:
    
    // (x-x0)*(v0-v) = (y-y0)*(u0-u) 
    //      => x*v0 - xv - x0*v0 + x0*v = y*u0 - yu - y0*u0 + y0*u 
    //      => xv - yu = v0*x - u0*y - y0*u + x0*v - x0*v0 + y0*u0

    // (x-x1)*(v1-v) = (y-y1)*(u1-u)
    //      => xv - yu = v1*x - u1*y - y1*u + x1*v - x1*v1 + y1*u1

    // and so we have...
    // v0*x - u0*y - y0*u + x0*v - x0*v0 + y0*u0 = v1*x - u1*y - y1*u + x1*v - x1*v1 + y1*u1
    //   => x(v0-v1) + y(u1-u0) + u(y1-y0) + v(x0-x1) = x0*v0 - y0*u0 + y1*u1 - x1*v1


    // we can construct 4 equations like that using 5 stones
    let ([x0, y0, z0], [u0, v0, w0]) = s[0];
    let ([x1, y1, z1], [u1, v1, w1]) = s[1];
    let ([x2, y2, z2], [u2, v2, w2]) = s[2];
    let ([x3, y3, _z3], [u3, v3, _w3]) = s[3];
    let ([x4, y4, _z4], [u4, v4, _w4]) = s[4];

    let a = Matrix4::new(
        v0-v1, u1-u0, y1-y0, x0-x1,
        v1-v2, u2-u1, y2-y1, x1-x2,
        v2-v3, u3-u2, y3-y2, x2-x3,
        v3-v4, u4-u3, y4-y3, x3-x4,
    );

    let b = Vector4::new(
        x0*v0 - y0*u0 + y1*u1 - x1*v1,
        x1*v1 - y1*u1 + y2*u2 - x2*v2,
        x2*v2 - y2*u2 + y3*u3 - x3*v3,
        x3*v3 - y3*u3 + y4*u4 - x4*v4,
    );

    let decomp = a.lu();
    let x = decomp.solve(&b).unwrap();
    let (x, y, u, _v) = (x[0], x[1], x[2], x[3]);

    // with x y u v we can now solve for z, w
    //      => xw - zu = w0*x - u0*z - z0*u + x0*w - x0*w0 + z0*u0
    // and so
    //      => z(u1-u0) + w(x0-x1) = x0*w0 - z0*u0 + z1*u1 - x1*w1 - x(w0-w1) - u(z1-z0)

    let a = Matrix2::new(
        u1-u0, x0-x1,
        u2-u1, x1-x2,
    );
    let b = Vector2::new(
        x0*w0 - z0*u0 + z1*u1 - x1*w1 - x*(w0-w1) - u*(z1-z0),
        x1*w1 - z1*u1 + z2*u2 - x2*w2 - x*(w1-w2) - u*(z2-z1),
    );

    let decomp = a.lu();
    let z = decomp.solve(&b).unwrap();
    let (z, _w) = (z[0], z[1]);

    // println!("{}, {}, {} @ {}, {}, {}", x, y, z, u, _v, _w);

    Some((x+y+z).round() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_impl(&advent_of_code::template::read_file("examples", DAY), [7.0, 27.0]);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
