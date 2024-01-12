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
#[derive(Default)]
pub struct System(Vec<Box<dyn Unit>>);

/// Numeric type for energy values. Energy is in arbitrary
/// units in this type.
type Energy = i32;

impl System {
    /// Put another unit into the system.
    pub fn add_unit(&mut self, u: Box<dyn Unit>) {
        self.0.push(u);
    }

    /// Find an optimal setpoint for the system, if any
    /// exists.
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

        let mut u_opt = self.find_energy(max);
        let mut t_opt = max;
        for t in min..max {
            let u = self.find_energy(t);
            if u < u_opt {
                u_opt = u;
                t_opt = t;
            }
        }

        Some(t_opt)
    }

    /// Find the total energy expended for reactions at this
    /// temperature.
    pub fn find_energy(&self, t: Temperature) -> Energy {
        let mut e = 0;
        for u in self.0.iter() {
            e += (t - u.nominal()).pow(2);
        }
        e
    }
}

#[test]
fn test_find_setpoint() {
    let mut system = System::default();
    assert!(system.find_setpoint().is_none());

    system.add_unit(Box::new(Heater { nominal: 20, max: 40 }));
    let setpoint = system.find_setpoint().unwrap();
    assert_eq!(20, setpoint);

    system.add_unit(Box::new(Cooler { nominal: 25, min: 15 }));
    let setpoint = system.find_setpoint().unwrap();
    assert!(setpoint == 22 || setpoint == 23);
    assert_eq!(system.find_energy(22), system.find_energy(23));

    system.add_unit(Box::new(HeaterCooler { nominal: 23, min: 0, max: 110 }));
    let setpoint = system.find_setpoint().unwrap();
    assert_eq!(23, setpoint);

    system.add_unit(Box::new(Heater { nominal: 25, max: 100 }));
    let setpoint = system.find_setpoint().unwrap();
    assert_eq!(25, setpoint);

    system.add_unit(Box::new(Cooler { nominal: 24, min: -100 }));
    assert!(system.find_setpoint().is_none());
}
