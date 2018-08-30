use std::{cmp, fmt};

//Closed: The boundary moment of time is included in calculations.
//Open: The boundary moment of time represents a boundary value which is excluded in regard to calculations.
#[derive(Debug, PartialEq, Clone)]
pub enum IntervalEdge {
    Closed,
    Open,
}

#[derive(Debug, Clone)]
pub struct Interval<T> {
    pub from: T,
    pub till: T,
    pub fromedge: IntervalEdge,
    pub tilledge: IntervalEdge,
}

impl<T> Interval<T>
where
    T: fmt::Display + cmp::PartialOrd + cmp::Eq,
{
    //Creates a new interval with specified edge types. Use closed/open_interval to construct completely open or closed intervals
    pub fn new(from: T, till: T, fromedge: IntervalEdge, tilledge: IntervalEdge) -> Interval<T> {
        assert!(from <= till
       , "invalid interval: interval is from {} till {}, while from should be smaller or equal than till."
       , from
       , till);

        Interval {
            from,
            till,
            fromedge,
            tilledge,
        }
    }

    //Constructs a closed interval [from,till]
    pub fn closed_interval(from: T, till: T) -> Interval<T> {
        Interval::new(from, till, IntervalEdge::Closed, IntervalEdge::Closed)
    }

    //Constructs an open interval (from,till)
    pub fn open_interval(from: T, till: T) -> Interval<T> {
        Interval::new(from, till, IntervalEdge::Open, IntervalEdge::Open)
    }

    //Constructs an open interval from an optional from and till.interval
    //if both are Some a Some(Interval) is constructed, otherwise None is returned
    //Note: use open_interval_from_opt or closed_interval_from_opt to construct completely open or closed intervals.
    pub fn interval_from_opt(
        optfrom: Option<T>,
        opttill: Option<T>,
        fromedge: IntervalEdge,
        tilledge: IntervalEdge,
    ) -> Option<Interval<T>> {
        if let (Some(from), Some(till)) = (optfrom, opttill) {
            return Some(Interval::new(from, till, fromedge, tilledge));
        }

        None
    }

    pub fn open_interval_from_opt(optfrom: Option<T>, opttill: Option<T>) -> Option<Interval<T>> {
        Interval::interval_from_opt(optfrom, opttill, IntervalEdge::Open, IntervalEdge::Open)
    }

    pub fn closed_interval_from_opt(optfrom: Option<T>, opttill: Option<T>) -> Option<Interval<T>> {
        Interval::interval_from_opt(optfrom, opttill, IntervalEdge::Closed, IntervalEdge::Closed)
    }

    fn contains_value(&self, value: &T, edge: &IntervalEdge) -> bool {
        if value > &self.from && value < &self.till {
            return true;
        } else if value == &self.from {
            return &self.fromedge == edge;
        } else if value == &self.till {
            return &self.tilledge == edge;
        }

        false
    }

    pub fn contains(&self, interval: &Interval<T>) -> bool {
        self.contains_value(&interval.from, &interval.fromedge)
            && self.contains_value(&interval.till, &interval.tilledge)
    }

    pub fn intersects(&self, other: &Interval<T>) -> bool {
        self.contains(other) || (other.from < self.from && self.till < other.till)
    }

    pub fn is_pointinterval(&self) -> bool {
        return self.from == self.till && self.is_closed();
    }

    pub fn is_from_closed(&self) -> bool {
        self.fromedge == IntervalEdge::Closed
    }

    pub fn is_from_open(&self) -> bool {
        self.fromedge == IntervalEdge::Open
    }

    pub fn is_till_closed(&self) -> bool {
        self.tilledge == IntervalEdge::Closed
    }

    pub fn is_till_open(&self) -> bool {
        self.tilledge == IntervalEdge::Open
    }

    pub fn is_closed(&self) -> bool {
        return self.is_from_closed() && self.is_till_closed();
    }

    pub fn is_open(&self) -> bool {
        return self.is_from_open() && self.is_till_open();
    }

    pub fn isempty(&self) -> bool {
        return self.from == self.till && !self.is_closed();
    }
}

impl<T> PartialEq for Interval<T>
where
    T: cmp::PartialEq,
{
    fn eq(&self, other: &Interval<T>) -> bool {
        self.from == other.from
            && self.till == other.till
            && self.fromedge == other.fromedge
            && self.tilledge == other.tilledge
    }
}
