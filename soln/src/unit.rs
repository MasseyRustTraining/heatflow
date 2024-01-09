//! Unit data for heatflow problem.

use crate::Temperature;

/// Types of reaction unit.
#[derive(Debug)]
pub enum Unit {
    Heater { nominal: Temperature, max: Temperature },
    Cooler { nominal: Temperature, min: Temperature },
}

impl Unit {
    /// Return a tuple of the minimum and maximum possible
    /// temperatures for this unit.
    pub fn limits(&self) -> (Temperature, Temperature) {
        match self {
            Unit::Heater { nominal, max } => (*nominal, *max),
            Unit::Cooler { nominal, min } => (*min, *nominal),
        }
    }
}
