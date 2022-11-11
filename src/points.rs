use serde_json::Value;

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

pub struct Error {
    pub cause: String
}

pub fn loadFigure(json_source: &str, figure:&mut Figure<3>) {

    fn toPoint(points: &Value) -> Result<[i64; 3], Error> { // move Option to Result
        match points {
            Value::Array(x) => if x.len() == 3 {
                let x0: &Value = &x[0];
                let x1: &Value = &x[1];
                let x2: &Value = &x[2];
                match (x0, x1, x2) {
                    (Value::Number(v1),
                        Value::Number(v2),
                        Value::Number(v3)) =>
                        match (v1.as_i64(),
                               v2.as_i64(),
                               v3.as_i64()) {
                            (Some(i1), Some(i2), Some(i3)) => Result::Ok([i1, i2, i3]),
                            _ => Result::Err(Error { cause: "Array size error".to_string() })
                        }

                    (_,_,_) => Result::Err(Error { cause: "Array size error".to_string() })
                }
            }
            else {
                Result::Err(Error { cause: "Array size error".to_string() })
            },
            _ => Result::Err(Error { cause: "Not an array".to_string() })
        }
    }
    fn loadPoints(figure:&mut Figure<3>, points: &Value) {
        match points {
            Value::Array(content) => {
                content.iter()
                    .map(|x| toPoint(x))
                    .for_each(|p | {
                        let fx = match p {
                            Result::Ok(x) => figure.add_point(x),
                            Result::Err(e) => {
                                panic!("{}", e.cause);
                            },
                        };
                    });
            },
            _ => panic!("points not an array")
        }
    }
    fn loadEdges(figure:&mut Figure<3>, edges: &Value) {
        match edges {
            Value::Array(content) => {
                content.iter()
                    .for_each(|edge | {
                        match edge {
                            Value::Array(content) => {
                                if content.len() != 2 {
                                    panic!("edge should have 2 ends")
                                }
                                let p1 = &content[0];
                                let p2 = &content[1];
                                match (p1, p2) {
                                    (Value::Number(point1), Value::Number(point2)) => {
                                        match (point1.as_u64(), point2.as_u64()) {
                                            (Some(u1), Some(u2)) => {
                                                figure.add_edge(u1 as usize, u2 as usize);
                                            },
                                            _ => panic!("end should be number")
                                        }
                                    },
                                    _ => panic!("edge ends should be number")
                                }
                            },
                            _ => panic!("edges not an array")
                        }
                    });
            },
            _ => panic!("edges not an array")
        }
    }
    figure.points.clear();
    figure.edges.clear();
    let json_figure = serde_json::from_str::<Value>(&json_source).unwrap();
    match json_figure {
        Value::Object(x) => {
            let points = x.get("points");
            match points {
                Some(pts) => loadPoints(figure, pts),
                None => panic!("not an object")
            }

            let edges = x.get("edges");
            match edges {
                Some(pts) => loadEdges(figure, pts),
                None => panic!("not an object")
            }
            figure.edges.iter().for_each(|e|  println!("edge : {} to {}", e.points.0, e.points.1));
        }
        _ => panic!("not an object")
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

    #[test]
    fn loadTest() {
        let figureDesc = "{
                \"points\": [
                   [10, 20, 10],
                   [-10, 0, -10],
                   [-10, 20, 0]
                ],
             \"edges\": [
                  [0,1], [1,2], [2,0]
            ]
        }";
        let mut fig: Figure<3> = Figure {
            points: vec![],
            edges: vec![]
        };
        loadFigure(&figureDesc, &mut fig);
        assert_eq!(3, fig.points.len());
        assert_eq!(3, fig.edges.len());
    }
}
pub trait Distance<T, const N: usize> {
    fn calc(from: T) -> f64;
}
