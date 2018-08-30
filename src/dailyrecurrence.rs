use datetimeinterval::*;
use interval::IntervalEdge;
use recurrence::*;
use timeinterval::TimeInterval;
use types::{Duration, Time, MIN_DATE};

pub struct DailyRecurrence {
    pub valid_interval: DateTimeInterval,
    pub time_interval: TimeInterval,
    pub repetition_interval: Duration,
}

impl DailyRecurrence {
    pub fn new(
        valid_interval: DateTimeInterval,
        time_interval: TimeInterval,
        repetition_days: i64,
    ) -> DailyRecurrence {
        assert!(
            repetition_days >= 1,
            "invalid repetition_interval: repetition_interval is 0, while it should be equal or larger than 1"
        );
        DailyRecurrence {
            valid_interval,
            time_interval,
            repetition_interval: Duration::days(repetition_days),
        }
    }
}

impl IntoIterator for DailyRecurrence {
    type Item = DateTimeInterval;
    type IntoIter = RecurrenceIterator<DailyRecurrence>;

    fn into_iter(self) -> Self::IntoIter {
        RecurrenceIterator::new(self)
    }
}

fn check_if_interval_in_valid_interval(
    optinterval: &Option<DateTimeInterval>,
    valid_interval: &DateTimeInterval,
) -> Option<DateTimeInterval> {
    if let Some(interval) = optinterval {
        if valid_interval.contains(interval) {
            return optinterval.clone();
        }
    }

    None
}

impl Recurrence for DailyRecurrence {
    fn first_interval(&self) -> Option<DateTimeInterval> {
        let startinterval = DateTimeInterval::new(
            MIN_DATE.and_time(Time::from_hms(0, 00, 00)).unwrap(),
            self.valid_interval.from.clone(),
            IntervalEdge::Closed,
            IntervalEdge::Closed,
        );
        return Some(startinterval);
    }

    fn get_next_interval(&self, interval: &DateTimeInterval) -> Option<DateTimeInterval> {
        if interval.from.time() == self.time_interval.from
            && interval.till.time() == self.time_interval.till
        {
            let from = interval.from.checked_add_signed(self.repetition_interval);
            let till = interval.till.checked_add_signed(self.repetition_interval);
            let newintervalopt = DateTimeInterval::interval_from_opt(
                from,
                till,
                self.time_interval.fromedge.clone(),
                self.time_interval.tilledge.clone(),
            );
            return check_if_interval_in_valid_interval(&newintervalopt, &self.valid_interval);
        }

        let mut numdays = self.repetition_interval;

        if interval.till.time() < self.time_interval.from {
            numdays = numdays - Duration::days(1);
        }

        let next_date_opt = interval.till.date().checked_add_signed(numdays);
        if let Some(next_date) = next_date_opt {
            let from = next_date.and_time(self.time_interval.from);
            let till = next_date.and_time(self.time_interval.till);
            let optnextinterval = DateTimeInterval::interval_from_opt(
                from,
                till,
                self.time_interval.fromedge.clone(),
                self.time_interval.tilledge.clone(),
            );

            return check_if_interval_in_valid_interval(&optnextinterval, &self.valid_interval);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;
    use chrono::Utc;
    use types::Time;

    fn basic_calendar() -> DailyRecurrence {
        let from = Time::from_hms(12, 00, 00);
        let till = Time::from_hms(13, 00, 00);
        let ti = TimeInterval::closed_interval(from, till);

        let valid_from = Utc.ymd(2018, 8, 1).and_time(Time::from_hms(0, 00, 00));
        let valid_till = Utc.ymd(2018, 9, 1).and_time(Time::from_hms(0, 00, 00));
        let validdi = DateTimeInterval::interval_from_opt(
            valid_from,
            valid_till,
            IntervalEdge::Closed,
            IntervalEdge::Open,
        );

        return DailyRecurrence::new(validdi.unwrap(), ti, 1);
    }

    #[test]
    fn first_interval() {
        let drec = basic_calendar();
        let expectedinterval = DateTimeInterval::closed_interval_from_opt(
            Utc.ymd(2018, 8, 1).and_time(Time::from_hms(12, 00, 00)),
            Utc.ymd(2018, 8, 1).and_time(Time::from_hms(13, 00, 00)),
        );

        assert_eq!(drec.into_iter().next(), expectedinterval);
    }

    #[test]
    fn second_interval() {
        let drec = basic_calendar();
        let expectedinterval = DateTimeInterval::closed_interval_from_opt(
            Utc.ymd(2018, 8, 2).and_time(Time::from_hms(12, 00, 00)),
            Utc.ymd(2018, 8, 2).and_time(Time::from_hms(13, 00, 00)),
        );

        assert_eq!(drec.into_iter().skip(1).next(), expectedinterval);
    }

    #[test]
    fn last_interval() {
        let drec = basic_calendar();
        let expectedinterval = DateTimeInterval::closed_interval_from_opt(
            Utc.ymd(2018, 8, 31).and_time(Time::from_hms(12, 00, 00)),
            Utc.ymd(2018, 8, 31).and_time(Time::from_hms(13, 00, 00)),
        );

        assert_eq!(drec.into_iter().skip(30).next(), expectedinterval);
    }

    #[test]
    fn one_beyond_last_interval() {
        let drec = basic_calendar();
        assert_eq!(drec.into_iter().skip(31).next(), None);
    }

    #[test]
    fn two_beyond_last_interval() {
        let drec = basic_calendar();
        assert_eq!(drec.into_iter().skip(32).next(), None);
    }
}
