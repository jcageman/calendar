use datetimeinterval::*;

pub trait Recurrence: IntoIterator {
    fn first_interval(&self) -> Option<DateTimeInterval>;
    fn get_next_interval(&self, interval: &DateTimeInterval) -> Option<DateTimeInterval>;
}

#[derive(Debug)]
pub struct RecurrenceIterator<T> {
    interval: Option<DateTimeInterval>,
    recurrence: T,
}

impl<T> RecurrenceIterator<T>
where
    T: Recurrence,
{
    pub fn new(recurrence: T) -> RecurrenceIterator<T> {
        let interval = recurrence.first_interval();
        RecurrenceIterator {
            interval,
            recurrence,
        }
    }
}

fn next_interval<T>(
    recurrence: &T,
    current_interval: &Option<DateTimeInterval>,
) -> Option<DateTimeInterval>
where
    T: Recurrence,
{
    if let Some(ref interval) = current_interval {
        return recurrence.get_next_interval(&interval);
    }

    None
}

impl<T> Iterator for RecurrenceIterator<T>
where
    T: Recurrence,
{
    type Item = DateTimeInterval;

    fn next(&mut self) -> Option<Self::Item> {
        self.interval = next_interval(&self.recurrence, &self.interval);
        self.interval.clone()
    }
}
