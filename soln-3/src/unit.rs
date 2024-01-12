//! Unit data for heatflow problem.

use crate::Temperature;

/// Heater reaction unit.
pub struct Heater {
    pub nominal: Temperature,
    pub max: Temperature,
}

/// Cooler reaction unit.
pub struct Cooler {
    pub nominal: Temperature,
    pub min: Temperature,
}

/// Heater/Cooler reaction unit.
pub struct HeaterCooler {
    pub nominal: Temperature,
    pub min: Temperature,
    pub max: Temperature,
}

pub trait Unit {
    /// Return a tuple of the minimum and maximum possible
    /// temperatures for this unit.
    fn limits(&self) -> (Temperature, Temperature);

    /// Return the nominal temperature for this unit.
    fn nominal(&self) -> Temperature;
}

impl Unit for Heater {
    fn limits(&self) -> (Temperature, Temperature) {
        (self.nominal, self.max)
    }

    fn nominal(&self) -> Temperature {
        self.nominal
    }
}

impl Unit for Cooler {
    fn limits(&self) -> (Temperature, Temperature) {
        (self.min, self.nominal)
    }

    fn nominal(&self) -> Temperature {
        self.nominal
    }
}

impl Unit for HeaterCooler {
    fn limits(&self) -> (Temperature, Temperature) {
        (self.min, self.max)
    }

    fn nominal(&self) -> Temperature {
        self.nominal
    }
}
