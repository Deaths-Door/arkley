use arkley_unit_convertor::*;
#[test]
fn test_seconds_to_minutes() {
    let time = Time::new(60.0, TimeUnits::Seconds);
    let converted = time.convert_to(TimeUnits::Minutes);
    assert_eq!(*converted.number(), 1.0);
}

#[test]
fn test_minutes_to_hours() {
    let time = Time::new(60.0, TimeUnits::Minutes);
    let converted = time.convert_to(TimeUnits::Hours);
    assert_eq!(*converted.number(), 1.0);
}

#[test]
fn test_hours_to_days() {
    let time = Time::new(24.0, TimeUnits::Hours);
    let converted = time.convert_to(TimeUnits::Days);
    assert_eq!(*converted.number(), 1.0);
}

#[test]
fn test_days_to_seconds() {
    let time = Time::new(1.0 / 24.0, TimeUnits::Days);
    let converted = time.convert_to(TimeUnits::Seconds);
    assert_eq!(*converted.number(), 3600.0);
}

#[test]
fn test_convert_back_to_original() {
    let time = Time::new(3600.0, TimeUnits::Seconds);
    let converted = time.to_minutes().to_hours().to_days().to_seconds();
    assert_eq!(*converted.number(), 3600.0);
}