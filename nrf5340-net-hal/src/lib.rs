#![no_std]

use embedded_hal as hal;
pub use nrf52_hal_common::*;
pub use nrf5340_net_pac;

pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use nrf52_hal_common::prelude::*;

    pub use crate::time::U32Ext;
}

pub use crate::clocks::Clocks;
pub use crate::delay::Delay;
pub use crate::spim::Spim;
pub use crate::timer::Timer;
pub use crate::uarte::Uarte;
