use std::fmt::{Result, Formatter, Display};
use TimeSpan;
use super::TimeFormatter;

pub struct Inner(Option<TimeSpan>);
pub struct Complete;

impl<'a> TimeFormatter<'a> for Complete {
    type Inner = Inner;

    fn format<T>(&self, time: T) -> Self::Inner
        where T: Into<Option<TimeSpan>>
    {
        Inner(time.into())
    }
}

impl Display for Inner {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(time) = self.0 {
            let mut total_seconds = time.total_seconds();
            if total_seconds < 0.0 {
                total_seconds *= -1.0;
                write!(f, "-")?;
            }
            let seconds = total_seconds % 60.0;
            let total_minutes = (total_seconds / 60.0) as u64;
            let minutes = total_minutes % 60;
            let hours = total_minutes / 60;
            write!(f, "{:02}:{:02}:{:010.7}", hours, minutes, seconds)
        } else {
            write!(f, "00:00:00.0000000")
        }
    }
}
