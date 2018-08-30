# calendar
rust library for calendar calculations
 - Builds up from chrono DateTime, Time and Date types
 - Supports time, date and datetime intervals (with a distinction between open and closed intervals, see: https://en.wikipedia.org/wiki/Interval_(mathematics))
 - Supports recurrence rules (for now only daily recurrence rules are supported)

Example 1 (defines a recurrence rule every day from 12:00-13:00. The rule is only valid from 2018-8-1 0:00:00 (closed) till 2018-9-31 0:00:00 (open).
```
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

let drec = DailyRecurrence::new(validdi.unwrap(), ti, 1);

let expectedinterval = DateTimeInterval::closed_interval_from_opt(
    Utc.ymd(2018, 8, 2).and_time(Time::from_hms(12, 00, 00)),
    Utc.ymd(2018, 8, 2).and_time(Time::from_hms(13, 00, 00)),
);

assert_eq!(drec.into_iter().skip(1).next(), expectedinterval);
```
