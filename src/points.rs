

//#[derive(Clone, Copy)]
pub struct Point<const N: usize> {
    pub coords: [i64; N],
}

pub struct Edge {
    pub points: (usize, usize),
}

pub struct Figure<const N: usize> {
    pub points: Vec<Point<N>>, // [&'a Point<N>; NP],
    pub edges: Vec<Edge>,
}

impl <const N: usize> Figure<N> {
    pub fn edge_points(&self, e: &Edge) -> (&Point<N>, &Point<N>) {
        return (&self.points[e.points.0],
                &self.points[e.points.1]);
    }
}


pub trait PointAdder<const N: usize> {
    fn add_point(&mut self, c: [i64; N]) -> &Self;
}

pub trait EdgeAdder {
    fn add_edge(&mut self, start: usize, end: usize)  -> &Self;
}

impl<const N: usize> PointAdder<N> for Figure<N> {
    fn add_point(&mut self, c: [i64; N]) -> &Self {
        let p : Point<N> = Point { coords : c };
        self.points.push(p);
        return self;
    }
}

impl<const N: usize> EdgeAdder for Figure<N> {
    fn add_edge(&mut self, start: usize, end: usize)  -> &Self {
        let edge = Edge { points: (start, end) };
        self.edges.push(edge);
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        println!("tests");
        let mut f : Figure<3> = Figure {
            points: vec![],
            edges: vec![]
        };
        f.add_point([3, 4, 5]);
        f.add_point([5, 7, 11]);
        let fig =  f.add_edge(0, 1);
        let e: &Edge = &fig.edges[0];
        let p: (&Point<3>, &Point<3>) = fig.edge_points(e);
        let p1 : &Point<3> = p.0;
        let p2 = p.1;
        assert_eq!(3, p1.coords[0]);
        assert_eq!(11, p2.coords[2]);
    }
}
pub trait Distance<T, const N: usize> {
    fn calc(from: T) -> f64;
}
/*
impl<'a, const N: usize> Distance<Point<N>, N> for Point<N> {
    fn calc(from: &'a Point<N>) -> f64 {
        let mut x: f64 = 0.0;
        for i in 0..N {
            x += from.coords[i].pow(2) as f64;
        }
        x.sqrt()
    }
}*/
/*
impl<const N: usize> Distance<Edge<N>, N> for Point<N> {
    fn calc(from: Edge<N>) -> f64 {
        0.0
    }
}
*/
