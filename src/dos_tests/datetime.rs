use rust_dos::{*, dos::datetime::{Date, Time}};

#[allow(dead_code)]
pub(crate) fn datetime_test() {
    let date = Date::now();
    let time = Time::now();

    println!("Today's date is {:?}", date);
    println!("Current time is {:?}", time);

    /* Commented out because this will set the hardware clock on DOS 3.3+
    let mut date = Date::now();
    let mut time = Time::now();

    date.day += 1;
    date.save().unwrap();
    let date2 = Date::now();
    println!("The date has been set to {:?}", date2);

    // Note that this does not work in DOSBox
    time.hour += 1;
    time.save().unwrap();
    let time2 = Time::now();
    println!("The time has been set to {:?}", time2);
    */
}