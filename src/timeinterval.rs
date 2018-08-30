use interval::*;
use types::{Duration, Time};

pub type TimeInterval = Interval<Time>;

impl TimeInterval {
    pub fn duration(&self) -> Duration {
        self.till - self.from
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use types::{Duration, Time};

    #[test]
    fn non_empty_interval() {
        let from = Time::from_hms(12, 00, 00);
        let till = Time::from_hms(13, 00, 00);
        let ti = TimeInterval::closed_interval(from, till);
        assert!(!ti.isempty());
        assert_ne!(ti.from, ti.till);
        assert_eq!(ti.duration(), Duration::hours(1))
    }

    #[test]
    fn empty_interval() {
        let from = Time::from_hms(12, 00, 00);
        let ti = TimeInterval::open_interval(from, from);
        assert!(ti.isempty());
        assert_eq!(ti.from, ti.till);
        assert_eq!(ti.duration(), Duration::zero());
    }

    #[test]
    #[should_panic(expected = "invalid interval")]
    fn from_larger_than_till() {
        let from = Time::from_hms(12, 00, 00);
        let till = from - Duration::nanoseconds(1);
        TimeInterval::closed_interval(from, till);
    }
}
