// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Client API for the STM32G0 SYS server.

#![no_std]

use unwrap_lite::UnwrapLite;
use userlib::*;

pub use drv_stm32xx_gpio_common::{
    Alternate, Mode, OutputType, PinSet, Port, Pull, Speed,
};

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum RccError {
    NoSuchPeripheral = 1,
}

impl TryFrom<u32> for RccError {
    type Error = ();
    fn try_from(x: u32) -> Result<Self, Self::Error> {
        match x {
            1 => Ok(RccError::NoSuchPeripheral),
            _ => Err(()),
        }
    }
}

impl From<RccError> for u16 {
    fn from(x: RccError) -> Self {
        x as u16
    }
}

impl Sys {
    /// Requests that the clock to a peripheral be turned on.
    ///
    /// This operation is idempotent and will be retried automatically should
    /// the RCC server crash while processing it.
    ///
    /// # Panics
    ///
    /// If the RCC server has died.
    pub fn enable_clock(&self, peripheral: Peripheral) {
        // We are unwrapping here because the RCC server should not return
        // NoSuchPeripheral for a valid member of the Peripheral enum.
        self.enable_clock_raw(peripheral as u32).unwrap_lite()
    }

    /// Requests that the clock to a peripheral be turned off.
    ///
    /// This operation is idempotent and will be retried automatically should
    /// the RCC server crash while processing it.
    ///
    /// # Panics
    ///
    /// If the RCC server has died.
    pub fn disable_clock(&self, peripheral: Peripheral) {
        // We are unwrapping here because the RCC server should not return
        // NoSuchPeripheral for a valid member of the Peripheral enum.
        self.disable_clock_raw(peripheral as u32).unwrap_lite()
    }

    /// Requests that the reset line to a peripheral be asserted.
    ///
    /// This operation is idempotent and will be retried automatically should
    /// the RCC server crash while processing it.
    ///
    /// # Panics
    ///
    /// If the RCC server has died.
    pub fn enter_reset(&self, peripheral: Peripheral) {
        // We are unwrapping here because the RCC server should not return
        // NoSuchPeripheral for a valid member of the Peripheral enum.
        self.enter_reset_raw(peripheral as u32).unwrap_lite()
    }

    /// Requests that the reset line to a peripheral be deasserted.
    ///
    /// This operation is idempotent and will be retried automatically should
    /// the RCC server crash while processing it.
    ///
    /// # Panics
    ///
    /// If the RCC server has died.
    pub fn leave_reset(&self, peripheral: Peripheral) {
        // We are unwrapping here because the RCC server should not return
        // NoSuchPeripheral for a valid member of the Peripheral enum.
        self.leave_reset_raw(peripheral as u32).unwrap_lite()
    }
}

/// Peripherals appear in "groups." All peripherals in a group are controlled
/// from the same subset of registers in the RCC.
///
/// The reference manual lacks a term for this, so we made this one up. It would
/// be tempting to refer to these as "buses," but in practice there are almost
/// always more groups than there are buses, particularly on M0.
///
/// This is `pub` mostly for use inside driver-servers.
#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Group {
    Iop = 0,
    Ahb,
    Apb1,
    Apb2,
}

/// Assign peripheral numbers that are unique by group.
const fn periph(g: Group, bit_number: u8) -> u32 {
    // Note: this will accept bit numbers higher than 31, and they'll wrap
    // around to zero. Asserting here would be nice, but asserts in const fns
    // are not yet stable. In practice, you are likely to get a compile error if
    // you make a mistake here, because it will cause enum variants to alias to
    // the same number which is not permitted.
    (g as u32) << 5 | (bit_number & 0x1F) as u32
}

/// Peripheral numbering.
///
/// Peripheral bit numbers per the STM32G0 documentation, starting at section:
///
///    STM32G0 PART     MANUAL      SECTION
///    G0x0             RM0454      5.4.8 (RCC_IOPRSTR)
///    G0x1             RM0444      5.4.9 (RCC_IOPRSTR)
///
/// These are in the order that they appear in the documentation.   This is
/// the union of all STM32G0 peripherals; not all peripherals will exist on
/// all variants!
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u32)]
pub enum Peripheral {
    GpioF = periph(Group::Iop, 5),
    GpioE = periph(Group::Iop, 4),
    GpioD = periph(Group::Iop, 3),
    GpioC = periph(Group::Iop, 2),
    GpioB = periph(Group::Iop, 1),
    GpioA = periph(Group::Iop, 0),

    Rng = periph(Group::Ahb, 18), // G0x1 only
    Aes = periph(Group::Ahb, 16), // G0x1 only
    Crc = periph(Group::Ahb, 12),
    Flash = periph(Group::Ahb, 8),
    Dma2 = periph(Group::Ahb, 1),
    Dma1 = periph(Group::Ahb, 0),

    LpTim1 = periph(Group::Apb1, 31), // G0x1 only
    LpTim2 = periph(Group::Apb1, 30), // G0x1 only
    Dac1 = periph(Group::Apb1, 29),   // G0x1 only
    Pwr = periph(Group::Apb1, 28),
    Dbg = periph(Group::Apb1, 27),
    Ucpd2 = periph(Group::Apb1, 26), // G0x1 only
    Ucpd1 = periph(Group::Apb1, 25), // G0x1 only
    Cec = periph(Group::Apb1, 24),   // G0x1 only
    I2c3 = periph(Group::Apb1, 23),
    I2c2 = periph(Group::Apb1, 22),
    I2c1 = periph(Group::Apb1, 21),
    LpUart1 = periph(Group::Apb1, 20), // G0x1 only
    Usart4 = periph(Group::Apb1, 19),
    Usart3 = periph(Group::Apb1, 18),
    Usart2 = periph(Group::Apb1, 17),
    Crs = periph(Group::Apb1, 16), // G0x1 only
    Spi3 = periph(Group::Apb1, 15),
    Spi2 = periph(Group::Apb1, 14),
    Usb = periph(Group::Apb1, 13),
    Fdcan = periph(Group::Apb1, 12), // G0x1 only
    Usart6 = periph(Group::Apb1, 9),
    Usart5 = periph(Group::Apb1, 8),
    LpUart2 = periph(Group::Apb1, 7), // G0x1 only
    Tim7 = periph(Group::Apb1, 5),
    Tim6 = periph(Group::Apb1, 4),
    Tim4 = periph(Group::Apb1, 2),
    Tim3 = periph(Group::Apb1, 1),
    Tim2 = periph(Group::Apb1, 0), // G0x1 only

    Adc = periph(Group::Apb2, 20),
    Tim17 = periph(Group::Apb2, 18),
    Tim16 = periph(Group::Apb2, 17),
    Tim15 = periph(Group::Apb2, 16),
    Tim14 = periph(Group::Apb2, 15),
    Usart1 = periph(Group::Apb2, 14),
    Spi1 = periph(Group::Apb2, 12),
    Tim1 = periph(Group::Apb2, 11),
    Syscfg = periph(Group::Apb2, 0),
}

impl Peripheral {
    #[inline(always)]
    pub fn group(self) -> Group {
        let index = (self as u32 >> 5) as u8;
        // Safety: this is unsafe because it can turn any arbitrary bit pattern
        // into a `Group`, potentially resulting in undefined behavior. However,
        // `self` is a valid `Peripheral`, and we make sure (above) that
        // `Peripheral` has valid values in its `Group` bits by only
        // constructing it _from_ a `Group`. So this is safe.
        //
        // The reason this is using unsafe code in the _first_ place is to
        // ensure that we don't generate an unnecessary panic here. We don't
        // need the panic because we already checked user input on the way into
        // the `Peripheral` type.
        unsafe { core::mem::transmute(index) }
    }

    #[inline(always)]
    pub fn bit_index(self) -> u8 {
        self as u8 & 0x1F
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum GpioError {
    BadArg = 2,
}

impl From<GpioError> for u32 {
    fn from(rc: GpioError) -> Self {
        rc as u32
    }
}

impl From<GpioError> for u16 {
    fn from(rc: GpioError) -> Self {
        rc as u16
    }
}

impl TryFrom<u32> for GpioError {
    type Error = ();

    fn try_from(x: u32) -> Result<Self, Self::Error> {
        match x {
            2 => Ok(GpioError::BadArg),
            _ => Err(()),
        }
    }
}

impl Sys {
    /// Configures a subset of pins in a GPIO port.
    ///
    /// This is the raw operation, which can be useful if you're doing something
    /// unusual, but see `configure_output`, `configure_input`, and
    /// `configure_alternate` for the common cases.
    pub fn gpio_configure(
        &self,
        port: Port,
        pins: u16,
        mode: Mode,
        output_type: OutputType,
        speed: Speed,
        pull: Pull,
        af: Alternate,
    ) -> Result<(), GpioError> {
        let packed_attributes = mode as u16
            | (output_type as u16) << 2
            | (speed as u16) << 3
            | (pull as u16) << 5
            | (af as u16) << 7;

        self.gpio_configure_raw(port, pins, packed_attributes)
    }

    /// Configures the pins in `PinSet` as high-impedance digital inputs, with
    /// optional pull resistors.
    pub fn gpio_configure_input(
        &self,
        pinset: PinSet,
        pull: Pull,
    ) -> Result<(), GpioError> {
        self.gpio_configure(
            pinset.port,
            pinset.pin_mask,
            Mode::Input,
            OutputType::PushPull, // doesn't matter
            Speed::High,          // doesn't matter
            pull,
            Alternate::AF0, // doesn't matter
        )
    }

    /// Configures the pins in `PinSet` as digital GPIO outputs, either
    /// push-pull or open-drain, with adjustable slew rate filtering and pull
    /// resistors.
    pub fn gpio_configure_output(
        &self,
        pinset: PinSet,
        output_type: OutputType,
        speed: Speed,
        pull: Pull,
    ) -> Result<(), GpioError> {
        self.gpio_configure(
            pinset.port,
            pinset.pin_mask,
            Mode::Output,
            output_type,
            speed,
            pull,
            Alternate::AF0, // doesn't matter
        )
    }

    /// Configures the pins in `PinSet` in the given alternate function.
    ///
    /// If the alternate function is an output, the `OutputType` and `Speed`
    /// settings apply. If it's an input, they don't matter; consider using
    /// `configure_alternate_input` in that case.
    pub fn gpio_configure_alternate(
        &self,
        pinset: PinSet,
        output_type: OutputType,
        speed: Speed,
        pull: Pull,
        af: Alternate,
    ) -> Result<(), GpioError> {
        self.gpio_configure(
            pinset.port,
            pinset.pin_mask,
            Mode::Alternate,
            output_type,
            speed,
            pull,
            af,
        )
    }

    /// Configures the pins in `PinSet` in the given alternate function, which
    /// should be an input.
    ///
    /// This calls `configure_alternate` passing arbitrary values for
    /// `OutputType` and `Speed`. This is appropriate for inputs, but not for
    /// outputs or bidirectional signals.
    pub fn gpio_configure_alternate_input(
        &self,
        pinset: PinSet,
        pull: Pull,
        af: Alternate,
    ) -> Result<(), GpioError> {
        self.gpio_configure_alternate(
            pinset,
            OutputType::OpenDrain,
            Speed::High,
            pull,
            af,
        )
    }

    /// Sets some pins high.
    pub fn gpio_set(&self, pinset: PinSet) -> Result<(), GpioError> {
        self.gpio_set_reset(pinset.port, pinset.pin_mask, 0)
    }

    /// Resets some pins low.
    pub fn gpio_reset(&self, pinset: PinSet) -> Result<(), GpioError> {
        self.gpio_set_reset(pinset.port, 0, pinset.pin_mask)
    }

    /// Sets some pins based on `flag` -- high if `true`, low if `false`.
    #[inline]
    pub fn gpio_set_to(
        &self,
        pinset: PinSet,
        flag: bool,
    ) -> Result<(), GpioError> {
        self.gpio_set_reset(
            pinset.port,
            if flag { pinset.pin_mask } else { 0 },
            if flag { 0 } else { pinset.pin_mask },
        )
    }

    pub fn gpio_read(&self, pinset: PinSet) -> Result<u16, GpioError> {
        Ok(self.gpio_read_input(pinset.port)? & pinset.pin_mask)
    }
}

include!(concat!(env!("OUT_DIR"), "/client_stub.rs"));
