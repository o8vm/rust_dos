use core::{arch::asm};

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
pub enum Day {
    Sunday = 0,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    UnknownDay,
}

impl From<u8> for Day {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Sunday,
            1 => Self::Monday,
            2 => Self::Tuesday,
            3 => Self::Wednesday,
            4 => Self::Thursday,
            5 => Self::Friday,
            6 => Self::Saturday,
            _ => Self::UnknownDay
        }
    }
}

impl Default for Day {
    fn default() -> Self {
        Self::Sunday
    }
}

#[derive(Default)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub day_of_week: Day
}

impl Date {
    pub fn now() -> Self {
        let mut date = Date::default();
        let mut day_of_week: u8;

        unsafe {
            asm!("mov ah, 0x2A",
                "int 0x21",
                out("cx") date.year,
                out("dh") date.month,
                out("dl") date.day,
                out("al") day_of_week);
        }
        
        date.day_of_week = Day::from(day_of_week);

        date
    }

    pub fn save(&self) -> Result<(), ()> {
        let mut result: u8;

        unsafe {
            asm!("mov ah, 0x2B",
                "int 0x21",
                in("cx") self.year,
                in("dh") self.month,
                in("dl") self.day,
                out("al") result);
        }

        if result == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl core::fmt::Debug for Date {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{:02}-{:02} ({:?})", self.year, self.month, self.day, self.day_of_week)
    }
}

#[derive(Default)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub centisecond: u8,
}

impl Time {
    pub fn now() -> Self {
        let mut time = Time::default();

        unsafe {
            asm!("mov ah, 0x2C",
                "int 0x21",
                out("ch") time.hour,
                out("cl") time.minute,
                out("dh") time.second,
                out("dl") time.centisecond);
        }

        time
    }

    pub fn save(&self) -> Result<(), ()> {
        let mut result: u8;

        unsafe {
            asm!("mov ah, 0x2D",
            "int 0x21",
            in("ch") self.hour,
            in("cl") self.minute,
            in("dh") self.second,
            in("dl") self.centisecond,
            out("al") result);
        }

        if result == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}

impl core::fmt::Debug for Time {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}.{:02}", self.hour, self.minute, self.second, self.centisecond)
    }
}
