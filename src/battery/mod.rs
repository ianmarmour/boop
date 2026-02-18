use std::{cell::RefCell, rc::Rc};

use battery::{Battery, Manager, units::Ratio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BatteryHandleError {
    #[error("no battery found for device")]
    BatteryNotFound,
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("an unknown error occured")]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct BatteryHandle {
    battery: Rc<RefCell<Battery>>,
    manager: Rc<RefCell<Manager>>, // must keep alive
}

impl BatteryHandle {
    pub fn new() -> Result<Self, BatteryHandleError> {
        let manager =
            battery::Manager::new().map_err(|e| BatteryHandleError::Internal(e.into()))?;

        let battery = match manager
            .batteries()
            .map_err(|e| BatteryHandleError::Internal(e.into()))?
            .next()
        {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                return Err(BatteryHandleError::Internal(e.into()));
            }
            None => {
                return Err(BatteryHandleError::BatteryNotFound);
            }
        };

        Ok(Self {
            battery: Rc::new(RefCell::new(battery)),
            manager: Rc::new(RefCell::new(manager)),
        })
    }

    pub fn refresh(&mut self) -> Result<(), BatteryHandleError> {
        let mut battery = self.battery.borrow_mut();

        self.manager
            .borrow()
            .refresh(&mut *battery)
            .map_err(|e| BatteryHandleError::Internal(e.into()))
    }

    pub fn charge(&self) -> Ratio {
        self.battery.borrow().state_of_charge()
    }
}
