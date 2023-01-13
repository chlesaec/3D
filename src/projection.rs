use eframe::epaint::{ Color32, Pos2 };
use std::ops::Div;

use crate::points::Point;

pub struct Screen {
    pub zoom: i64,
    pub projectionpos: i64,
    pub center: Point<2>,
    pub color: Color32,
    pub size: [u32; 2],
}

pub trait Projection<const N: usize> {
    fn project(&self, point: &Point<N>) -> Point<2>;
}

pub trait ToRealScreen {
    fn place(&self, point2D: &Point<2>) -> Pos2;
}

impl<'a, const N: usize> Projection<N> for Screen {
    fn project(&self, point: &Point<N>) -> Point<2> {
        fn transform_coord(screen: &Screen, distance: i64, v: i64) -> i64 {
            return (v * screen.zoom).div(screen.zoom + distance - screen.projectionpos);
        }
        fn apply_tranform<const N: usize>(screen: &Screen, p: &Point<N>, n: usize, v: i64) -> i64 {
            let mut result = v;
            for i in 2..n {
                result = transform_coord(screen, p.coords[i], result);
            }
            return result;
        }
        let x1 = apply_tranform(self, point, N, point.coords[0] - self.center.coords[0]);
        let x2 = apply_tranform(self, point, N, point.coords[1] - self.center.coords[1]);
        Point { coords: [x1, x2] }
    }
}

impl ToRealScreen for Screen {
    fn place(&self, point_2d: &Point<2>) -> Pos2 {
        let abs_point: Point<2> = Point {
            coords: [
                point_2d.coords[0] + self.center.coords[0],
                point_2d.coords[1] + self.center.coords[1],
            ],
        };
        return Pos2::new(
            (abs_point.coords[0] + (self.size[0] as i64 / 2)) as f32,
            ((self.size[1] as i64 / 2) - abs_point.coords[1]) as f32,
        );
    }
}
