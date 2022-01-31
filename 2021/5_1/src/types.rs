use regex::Regex;


// Point structure
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point{x: x, y: y}
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}


// Segment structure
pub struct Segment {
    start: Point,
    end: Point
}

impl Segment {
    pub fn new(x1: u16, y1: u16, x2: u16, y2: u16) -> Segment {
        Segment{start: Point::new(x1, y1), end: Point::new(x2, y2)}
    }

    pub fn from_line(line: String) -> Result<Segment, ()> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
        }
        let matches = REGEX.captures(&line).unwrap();
        let (x1, y1, x2, y2) = (
            &matches["x1"].parse::<u16>().unwrap(),
            &matches["y1"].parse::<u16>().unwrap(),
            &matches["x2"].parse::<u16>().unwrap(),
            &matches["y2"].parse::<u16>().unwrap()
        );
        if x1 != x2 && y1 != y2 {
            Err(())
        } else {
            Ok(Segment::new(*x1, *y1, *x2, *y2))
        }
    }

    pub fn get_points(&self) -> Vec<Point> {
        match self.start.x == self.end.x {
            true  => ((self.start.y)..(self.end.y + 1)).map(|y| Point{x: self.end.x, y: y}).collect::<Vec<Point>>(),
            false => ((self.start.x)..(self.end.x + 1)).map(|x| Point{x: x, y: self.end.y}).collect::<Vec<Point>>()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{{\n  start => {},\n  end   => {}\n}}", self.start.to_string(), self.end.to_string())
    }
}

// Matrix structure
pub struct Matrix {
    values: Vec<Vec<u16>>
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix{values: vec![vec![0; 1000]; 1000]}
    }

    pub fn count_crowded(&self) -> u16 {
        self.values
            .iter()
            .map(|row| row.into_iter().filter(|&&cell| cell > 1).count() as u16)
            .sum::<u16>()
    }

    fn mark_point(&mut self, point: Point) {
        self.values[point.x as usize][point.y as usize] += 1;
    }

    fn mark_points(&mut self, points: Vec<Point>) {
        points.into_iter().for_each(|point| self.mark_point(point));
    }

    pub fn mark_segment(&mut self, segment: Segment) {
        self.mark_points(segment.get_points())
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.values)
    }
}
