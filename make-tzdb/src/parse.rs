use std::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) enum TimeZone {
    TimeZone {
        transitions: Vec<Transition>,
        local_time_types: Vec<LocalTimeType>,
        leap_seconds: Vec<LeapSecond>,
        extra_rule: Option<TransitionRule>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum Transition {
    Transition {
        unix_leap_time: i64,
        local_time_type_index: usize,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum LeapSecond {
    LeapSecond {
        unix_leap_time: i64,
        correction: i32,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum TransitionRule {
    Fixed(LocalTimeType),
    Alternate(AlternateTime),
}

#[derive(Debug, Deserialize)]
pub(crate) enum AlternateTime {
    AlternateTime {
        std: LocalTimeType,
        dst: LocalTimeType,
        dst_start: RuleDay,
        dst_start_time: i32,
        dst_end: RuleDay,
        dst_end_time: i32,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum LocalTimeType {
    LocalTimeType {
        ut_offset: i32,
        is_dst: bool,
        time_zone_designation: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
pub(crate) enum RuleDay {
    Julian1WithoutLeap(Julian1WithoutLeap),
    Julian0WithLeap(Julian0WithLeap),
    MonthWeekDay(MonthWeekDay),
}

#[derive(Debug, Deserialize)]
pub(crate) enum Julian1WithoutLeap {
    Julian1WithoutLeap(u16),
}

#[derive(Debug, Deserialize)]
pub(crate) enum Julian0WithLeap {
    Julian0WithLeap(u16),
}

#[derive(Debug, Deserialize)]
pub(crate) enum MonthWeekDay {
    MonthWeekDay { month: u8, week: u8, week_day: u8 },
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Transition::Transition {
            unix_leap_time,
            local_time_type_index,
        } = &self;
        writeln!(
            f,
            "crate::generated::new_transition({}, {})",
            unix_leap_time, local_time_type_index
        )?;
        Ok(())
    }
}

impl fmt::Display for LocalTimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LocalTimeType::LocalTimeType {
            ut_offset,
            is_dst,
            time_zone_designation,
        } = self;
        let time_zone_designation = time_zone_designation.as_deref().map(DisplayTzd);
        writeln!(
            f,
            "crate::generated::new_local_time_type({}, {}, {})",
            ut_offset,
            is_dst,
            DisplayOption(time_zone_designation.as_ref()),
        )?;
        Ok(())
    }
}

impl fmt::Display for LeapSecond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LeapSecond::LeapSecond {
            unix_leap_time,
            correction,
        } = self;
        writeln!(
            f,
            "crate::generated::new_leap_second({}, {})",
            unix_leap_time, correction
        )?;
        Ok(())
    }
}

impl fmt::Display for TransitionRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransitionRule::Fixed(t) => writeln!(f, "TransitionRule::Fixed({})", t)?,
            TransitionRule::Alternate(t) => {
                writeln!(f, "TransitionRule::Alternate({})", t)?;
            },
        }
        Ok(())
    }
}

impl fmt::Display for AlternateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let AlternateTime::AlternateTime {
            std,
            dst,
            dst_start,
            dst_start_time,
            dst_end,
            dst_end_time,
        } = self;
        writeln!(
            f,
            "crate::generated::new_alternate_time({}, {}, {}, {}, {}, {})",
            std, dst, dst_start, dst_start_time, dst_end, dst_end_time,
        )
    }
}

impl fmt::Display for MonthWeekDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MonthWeekDay::MonthWeekDay {
            month,
            week,
            week_day,
        } = self;
        writeln!(
            f,
            "crate::generated::new_month_week_day({}, {}, {})",
            month, week, week_day
        )
    }
}

impl fmt::Display for Julian0WithLeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Julian0WithLeap::Julian0WithLeap(t) = self;
        writeln!(f, "crate::generated::new_julian0_with_leap({})", t)
    }
}

impl fmt::Display for Julian1WithoutLeap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Julian1WithoutLeap::Julian1WithoutLeap(t) = self;
        writeln!(f, "crate::generated::new_julian1_without_leap({})", t)
    }
}

impl fmt::Display for RuleDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleDay::Julian0WithLeap(t) => writeln!(f, "RuleDay::Julian0WithLeap({})", t)?,
            RuleDay::Julian1WithoutLeap(t) => writeln!(f, "RuleDay::Julian1WithoutLeap({})", t)?,
            RuleDay::MonthWeekDay(t) => {
                writeln!(f, "RuleDay::MonthWeekDay({})", t)?;
            },
        }
        Ok(())
    }
}

pub(crate) struct DisplayVec<'a, T>(pub(crate) &'a [T]);

impl<'a, T: fmt::Display> fmt::Display for DisplayVec<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        for elem in self.0 {
            writeln!(f, "    {},", elem)?;
        }
        writeln!(f, "]")
    }
}

pub(crate) struct DisplayOption<'a, T>(Option<&'a T>);

impl<'a, T: fmt::Display> fmt::Display for DisplayOption<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(v) => writeln!(f, "Some({})", v),
            None => writeln!(f, "None"),
        }
    }
}

struct DisplayTzd<'a>(&'a str);

impl fmt::Display for DisplayTzd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b{:?}", &self.0)
    }
}

impl fmt::Display for TimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TimeZone::TimeZone {
            transitions,
            local_time_types,
            leap_seconds,
            extra_rule,
        } = self;
        writeln!(
            f,
            "crate::generated::new_time_zone_ref(&{}, &{}, &{}, &{})",
            DisplayVec(transitions),
            DisplayVec(local_time_types),
            DisplayVec(leap_seconds),
            DisplayOption(extra_rule.as_ref()),
        )
    }
}
