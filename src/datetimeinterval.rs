use interval::*;
use types::{DateTime, Duration};

pub type DateTimeInterval = Interval<DateTime>;

impl DateTimeInterval {
    pub fn duration(&self) -> Duration {
        self.till - self.from
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use types::Duration;

    #[test]
    fn non_empty_interval() {
        let from = Utc::now();
        let till = from + Duration::nanoseconds(1);
        let dti = DateTimeInterval::closed_interval(from, till);
        assert!(!dti.isempty());
        assert_ne!(dti.from, dti.till);
        assert_eq!(dti.duration(), Duration::nanoseconds(1))
    }

    #[test]
    fn empty_interval() {
        let from = Utc::now();
        let dti = DateTimeInterval::open_interval(from, from);
        assert!(dti.isempty());
        assert_eq!(dti.from, dti.till);
        assert_eq!(dti.duration(), Duration::zero());
    }

    #[test]
    #[should_panic(expected = "invalid interval")]
    fn from_larger_than_till() {
        let from = Utc::now();
        let till = from - Duration::nanoseconds(1);
        DateTimeInterval::closed_interval(from, till);
    }
}
