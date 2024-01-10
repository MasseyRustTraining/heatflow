//! Heatflow problem  
//! Bart Massey 2024-01

mod unit;
use unit::*;

/// Numeric type for temperature values. Temperatures are
/// represented in °C in this type.
pub type Temperature = i32;


/// A system of chemical reactions; a collection of units.
/// Note that a new empty system can be created with
/// `System::default()`.
#[derive(Debug, Default)]
pub struct System(Vec<Unit>);

impl System {
    /// Put another unit into the system.
    pub fn add_unit(&mut self, u: Unit) {
        self.0.push(u);
    }

    /// Find a setpoint for the system, if any exists.
    pub fn find_setpoint(&self) -> Option<Temperature>  {
        let mut us = self.0.iter();
        let (mut min, mut max) = us.next()?.limits();
        for u in us {
            let (min0, max0) = u.limits();
            min = min.max(min0);
            max = max.min(max0);
            if min > max {
                return None;
            }
        }
        Some(min)
    }
}

#[test]
fn test_find_setpoint() {
    use Unit::*;

    let mut system = System::default();
    assert!(system.find_setpoint().is_none());

    system.add_unit(Heater { nominal: 20, max: 40 });
    let setpoint = system.find_setpoint().unwrap();
    assert!(20 <= setpoint && setpoint <= 40);

    system.add_unit(Cooler { nominal: 25, min: 15 });
    let setpoint = system.find_setpoint().unwrap();
    assert!(20 <= setpoint && setpoint <= 25);

    system.add_unit(Heater { nominal: 25, max: 100 });
    let setpoint = system.find_setpoint().unwrap();
    assert_eq!(25, setpoint);

    system.add_unit(Cooler { nominal: 24, min: -100 });
    assert!(system.find_setpoint().is_none());
}
