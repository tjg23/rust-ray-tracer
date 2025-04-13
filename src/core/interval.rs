use std::ops::{Add, Range};

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
}
impl Add<f64> for Interval {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        Self {
            start: self.start + rhs,
            end: self.end + rhs,
        }
    }
}

impl Interval {
    pub fn empty() -> Self {
        Self {
            start: f64::INFINITY,
            end: f64::NEG_INFINITY,
        }
    }
    pub fn universe() -> Self {
        Self {
            start: f64::NEG_INFINITY,
            end: f64::INFINITY,
        }
    }
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            start: min,
            end: max,
        }
    }
    pub fn from_range(range: Range<f64>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
    pub fn from_pair(a: Interval, b: Interval) -> Self {
        Self {
            start: a.start.min(b.start),
            end: a.end.max(b.end),
        }
    }

    pub fn size(&self) -> f64 {
        self.end - self.start
    }

    pub fn contains(&self, x: f64) -> bool {
        self.start <= x && x <= self.end
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.start < x && x < self.end
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.start {
            self.start
        } else if x > self.end {
            self.end
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        Self {
            start: self.start - delta,
            end: self.end + delta,
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        start < end
    }
}
