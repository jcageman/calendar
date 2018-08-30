use interval::*;
use types::{Date, Duration};

pub type DateInterval = Interval<Date>;

impl DateInterval {
    pub fn duration(&self) -> Duration {
        self.till - self.from
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::Utc;
    use types::Duration;

    #[test]
    fn non_empty_interval() {
        let from = Utc.ymd(2018, 8, 26);
        let till = from.succ();
        let di = DateInterval::closed_interval(from, till);
        assert!(!di.isempty());
        assert_ne!(di.from, di.till);
        assert_eq!(di.duration(), Duration::days(1))
    }

    #[test]
    fn empty_interval() {
        let from = Utc.ymd(2018, 8, 26);
        let di = DateInterval::open_interval(from, from);
        assert!(di.isempty());
        assert_eq!(di.from, di.till);
        assert_eq!(di.duration(), Duration::zero());
    }

    #[test]
    #[should_panic(expected = "invalid interval")]
    fn from_larger_than_till() {
        let from = Utc.ymd(2018, 8, 26);
        let till = from.pred();
        DateInterval::closed_interval(from, till);
    }
}
