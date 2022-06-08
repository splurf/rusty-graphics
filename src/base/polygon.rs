pub use sdl2::{pixels::Color, rect::Point};
use {
    super::math::{Function, Variable},
    crate::{error::Result, math::FunctionType},
    std::vec::IntoIter,
};

#[derive(Clone, Debug)]
pub struct Polygon(Vec<Point>);

impl Polygon {
    pub fn new<A: Into<Point>, T: IntoIterator<Item = A>>(iter: T) -> Result<Self> {
        //  Convert each value into a valid `Point`
        let mut points = iter.into_iter().map(Into::into).collect::<Vec<Point>>();

        //  Remove any consecutive points
        points.dedup();

        //  Establish the length of the points in case of deduplication
        let n = points.len();

        //  A valid polygon requires 3 or more normal coordinates
        if n > 2 {
            let head = points[0];

            //  Connect the head to the tail if not already connected
            if head != points[n - 1] {
                points.push(head)
            }
            Ok(Polygon(points))
        } else {
            Err("Not enough points (Required 3 or more)".into())
        }
    }

    /**
     * The number of verticies
     */
    pub fn len(&self) -> usize {
        self.0.len() - 1
    }

    /**
     * The points of the polygon excluding the tail connector
     */
    pub fn points(&self) -> Vec<Point> {
        self.0[..self.len()].to_vec()
    }

    /**
     * Return the functions representing the lines between each point
     */
    pub fn functions(&self, ignore: Option<Variable>) -> Vec<Function> {
        let a = self.points();

        let mut b = self.points();
        b.remove(0);
        b.push(a[0]);

        let pairs = a.into_iter().zip(b.into_iter());

        if let Some(v) = ignore {
            pairs
                .filter_map(|points| {
                    let f = Function::from(points);

                    if let FunctionType::Constant { var, .. } = &f.function() {
                        (var != &v).then(|| f)
                    } else {
                        Some(f)
                    }
                })
                .collect()
        } else {
            pairs.map(Function::from).collect()
        }
    }

    /**
     * A helper method for calculating the intersections between the polygon and the provided value on the specified based on the specified `Variable`
     */
    pub fn intersection_at(&self, v: Variable, n: i32, ignore: Option<Variable>) -> Vec<Point> {
        let mut intersections = self
            .functions(ignore)
            .into_iter()
            .filter_map(|f| {
                let i = f.solve(v, n)?;
                let point = {
                    if v.is_x() {
                        (n, i)
                    } else {
                        (i, n)
                    }
                };
                f.within(point).then(|| point.into())
            })
            .collect::<Vec<Point>>();
        intersections.sort_by_key(|p| if v.is_x() { p.y() } else { p.x() });
        intersections
    }

    /**
     * Calculate any x-value intersections of the points along the slopes connecting each `Point` then return the inner points of connection only
     */
    pub fn intersections_at_x(&self, n: i32) -> Vec<Point> {
        self.intersection_at(Variable::X, n, Some(Variable::X))
    }

    /**
     * Calculate any y-value intersections of the points along the slopes connecting each `Point` then return the inner points of connection only
     */
    pub fn intersections_at_y(&self, n: i32) -> Vec<Point> {
        self.intersection_at(Variable::Y, n, Some(Variable::Y))
    }

    /**
     * A helper method for calculating a limit based on the provided bounds and comparison function pointers
     */
    fn limit(&self, bound: fn(Point) -> i32, cmp: fn(&i32, &i32) -> bool) -> i32 {
        let mut points = self.points();
        let mut limit = bound(points.remove(0));

        points.into_iter().map(bound).for_each(|n| {
            if cmp(&n, &limit) {
                limit = n
            }
        });
        limit
    }

    /**
     * The left-most point of the polygon
     */
    pub fn x_min(&self) -> i32 {
        self.limit(Point::x, i32::lt)
    }

    /**
     * The right-most point of the polygon
     */
    pub fn x_max(&self) -> i32 {
        self.limit(Point::x, i32::gt)
    }

    /**
     * The highest point of the polygon
     */
    pub fn y_min(&self) -> i32 {
        self.limit(Point::y, i32::lt)
    }

    /**
     * The lowest point of the polygon
     */
    pub fn y_max(&self) -> i32 {
        self.limit(Point::y, i32::gt)
    }
}

impl<'a> From<&'a Polygon> for &'a [Point] {
    fn from(polygon: &'a Polygon) -> Self {
        &polygon.0
    }
}

impl IntoIterator for &Polygon {
    type Item = Point;

    type IntoIter = IntoIter<Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}
