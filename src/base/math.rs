use {sdl2::rect::Point, std::fmt::Display};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Variable {
    X,
    Y,
}

impl Variable {
    pub fn is_x(&self) -> bool {
        self == &Self::X
    }

    pub fn is_y(&self) -> bool {
        self == &Self::Y
    }
}

impl ToString for Variable {
    fn to_string(&self) -> String {
        match self {
            Variable::X => "x",
            Variable::Y => "y",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Interval {
    x: (i32, i32),
    y: (i32, i32),
}

impl Interval {
    const fn new(x: (i32, i32), y: (i32, i32)) -> Self {
        Self {
            x: {
                if x.0 > x.1 {
                    (x.1, x.0)
                } else {
                    x
                }
            },
            y: {
                if y.0 > y.1 {
                    (y.1, y.0)
                } else {
                    y
                }
            },
        }
    }

    fn within<T>(&self, point: T) -> bool
    where
        Point: From<T>,
    {
        let point = Point::from(point);
        let (x, y) = (point.x(), point.y());
        (x >= self.x.0 && x <= self.x.1) && (y >= self.y.0 && y <= self.y.1)
    }
}

#[derive(Debug)]
pub enum FunctionType {
    //  could create FunctionType as enum then place that as field into Function struct which could also have an interval field
    Constant { var: Variable, val: i32 },
    Linear { m: f32, b: f32 },
}

impl FunctionType {
    /** Return the evalulation of plugging in the provided `i32` value in relation to the specified `Variable` */
    fn solve(&self, v: Variable, n: i32) -> Option<i32> {
        match self {
            Self::Constant { var, val } => {
                if var == &v {
                    (val == &n).then(|| n) //  this is technically inf solutions
                } else {
                    Some(*val)
                }
            }
            Self::Linear { m, b } => Some(match v {
                Variable::X => m * (n as f32) + b,
                Variable::Y => ((n as f32) - b) / m,
            } as i32),
        }
    }

    fn equal<T: Into<(i32, i32)>>(&self, point: T) -> bool {
        let (x, y) = point.into();

        match self {
            Self::Constant { var, val } => match var {
                Variable::X => &x == val,
                Variable::Y => &y == val,
            },
            Self::Linear { m, b } => y == ((m * x as f32) + b) as i32,
        }
    }
}

#[derive(Debug)]
pub struct Function {
    function: FunctionType,
    interval: Interval,
}

impl Function {
    const fn new(function: FunctionType, interval: Interval) -> Self {
        Self { function, interval }
    }

    pub const fn function(&self) -> &FunctionType {
        &self.function
    }

    pub const fn interval(&self) -> &Interval {
        &self.interval
    }

    pub fn within<T>(&self, point: T) -> bool
    where
        Point: From<T>,
    {
        self.interval.within(point)
    }

    pub fn solve(&self, v: Variable, n: i32) -> Option<i32> {
        self.function.solve(v, n)
    }

    pub fn equal<T: Into<(i32, i32)>>(&self, point: T) -> bool {
        self.function.equal(point)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} , [{} < x < {}] , [{} < y < {}]",
            match self.function {
                FunctionType::Constant { var, val } => format!("{} = {}", var.to_string(), val),
                FunctionType::Linear { m, b } => {
                    let mut data = "f(x) = ".to_owned();

                    let b_is_normal = b.is_normal();

                    if m != 0.0 {
                        if m != 1.0 {
                            data.push_str(&m.to_string())
                        }
                        data.push('x');

                        if b_is_normal {
                            data.push_str(&format!(
                                " {} ",
                                if b.is_sign_positive() { '+' } else { '-' }
                            ))
                        }
                    }
                    if b_is_normal {
                        data.push_str(&b.abs().to_string());
                    }
                    data
                }
            },
            self.interval.x.0,
            self.interval.x.1,
            self.interval.y.0,
            self.interval.y.1
        ))
    }
}

impl<T: Into<Point>> From<(T, T)> for Function {
    fn from(pair: (T, T)) -> Self {
        let (p1, p2) = (pair.0.into(), pair.1.into());

        let (x1, y1) = (p1.x(), p1.y());
        let (x2, y2) = (p2.x(), p2.y());

        Self::new(
            if x1 == x2 {
                FunctionType::Constant {
                    var: Variable::X,
                    val: x1,
                }
            } else if y1 == y2 {
                FunctionType::Constant {
                    var: Variable::Y,
                    val: y1,
                }
            } else {
                let m = ((y2 - y1) as f32) / ((x2 - x1) as f32);
                let b = (y1 as f32) - (m * (x1 as f32));

                FunctionType::Linear { m, b }
            },
            Interval::new((x1, x2), (y1, y2)),
        )
    }
}
