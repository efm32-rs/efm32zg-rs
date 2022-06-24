#![doc = "Peripheral access API for EFM32ZG210F16 microcontrollers (generated using svd2rust v0.24.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn DMA();
    fn GPIO_EVEN();
    fn TIMER0();
    fn ACMP0();
    fn ADC0();
    fn I2C0();
    fn GPIO_ODD();
    fn TIMER1();
    fn USART1_RX();
    fn USART1_TX();
    fn LEUART0();
    fn PCNT0();
    fn RTC();
    fn CMU();
    fn VCMP();
    fn MSC();
    fn AES();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 17] = [
    Vector { _handler: DMA },
    Vector {
        _handler: GPIO_EVEN,
    },
    Vector { _handler: TIMER0 },
    Vector { _handler: ACMP0 },
    Vector { _handler: ADC0 },
    Vector { _handler: I2C0 },
    Vector { _handler: GPIO_ODD },
    Vector { _handler: TIMER1 },
    Vector {
        _handler: USART1_RX,
    },
    Vector {
        _handler: USART1_TX,
    },
    Vector { _handler: LEUART0 },
    Vector { _handler: PCNT0 },
    Vector { _handler: RTC },
    Vector { _handler: CMU },
    Vector { _handler: VCMP },
    Vector { _handler: MSC },
    Vector { _handler: AES },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - DMA"]
    DMA = 0,
    #[doc = "1 - GPIO_EVEN"]
    GPIO_EVEN = 1,
    #[doc = "2 - TIMER0"]
    TIMER0 = 2,
    #[doc = "3 - ACMP0"]
    ACMP0 = 3,
    #[doc = "4 - ADC0"]
    ADC0 = 4,
    #[doc = "5 - I2C0"]
    I2C0 = 5,
    #[doc = "6 - GPIO_ODD"]
    GPIO_ODD = 6,
    #[doc = "7 - TIMER1"]
    TIMER1 = 7,
    #[doc = "8 - USART1_RX"]
    USART1_RX = 8,
    #[doc = "9 - USART1_TX"]
    USART1_TX = 9,
    #[doc = "10 - LEUART0"]
    LEUART0 = 10,
    #[doc = "11 - PCNT0"]
    PCNT0 = 11,
    #[doc = "12 - RTC"]
    RTC = 12,
    #[doc = "13 - CMU"]
    CMU = 13,
    #[doc = "14 - VCMP"]
    VCMP = 14,
    #[doc = "15 - MSC"]
    MSC = 15,
    #[doc = "16 - AES"]
    AES = 16,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "AES"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const aes::RegisterBlock = 0x400e_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AES {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AES").finish()
    }
}
#[doc = "AES"]
pub mod aes;
#[doc = "DMA"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dma::RegisterBlock = 0x400c_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        Self::PTR
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DMA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA").finish()
    }
}
#[doc = "DMA"]
pub mod dma;
#[doc = "MSC"]
pub struct MSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MSC {}
impl MSC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const msc::RegisterBlock = 0x400c_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const msc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MSC {
    type Target = msc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSC").finish()
    }
}
#[doc = "MSC"]
pub mod msc;
#[doc = "EMU"]
pub struct EMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMU {}
impl EMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const emu::RegisterBlock = 0x400c_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for EMU {
    type Target = emu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMU").finish()
    }
}
#[doc = "EMU"]
pub mod emu;
#[doc = "RMU"]
pub struct RMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RMU {}
impl RMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rmu::RegisterBlock = 0x400c_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rmu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RMU {
    type Target = rmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMU").finish()
    }
}
#[doc = "RMU"]
pub mod rmu;
#[doc = "CMU"]
pub struct CMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMU {}
impl CMU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cmu::RegisterBlock = 0x400c_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmu::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CMU {
    type Target = cmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CMU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMU").finish()
    }
}
#[doc = "CMU"]
pub mod cmu;
#[doc = "TIMER0"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer0::RegisterBlock = 0x4001_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0").finish()
    }
}
#[doc = "TIMER0"]
pub mod timer0;
#[doc = "TIMER1"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer1::RegisterBlock = 0x4001_0400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER1 {
    type Target = timer1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1").finish()
    }
}
#[doc = "TIMER1"]
pub mod timer1;
#[doc = "ACMP0"]
pub struct ACMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP0 {}
impl ACMP0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const acmp0::RegisterBlock = 0x4000_1000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acmp0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ACMP0 {
    type Target = acmp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ACMP0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACMP0").finish()
    }
}
#[doc = "ACMP0"]
pub mod acmp0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usart1::RegisterBlock = 0x4000_c400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART1").finish()
    }
}
#[doc = "USART1"]
pub mod usart1;
#[doc = "PRS"]
pub struct PRS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PRS {}
impl PRS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const prs::RegisterBlock = 0x400c_c000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const prs::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PRS {
    type Target = prs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PRS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRS").finish()
    }
}
#[doc = "PRS"]
pub mod prs;
#[doc = "IDAC0"]
pub struct IDAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IDAC0 {}
impl IDAC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const idac0::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const idac0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IDAC0 {
    type Target = idac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IDAC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDAC0").finish()
    }
}
#[doc = "IDAC0"]
pub mod idac0;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x4000_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "VCMP"]
pub struct VCMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VCMP {}
impl VCMP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const vcmp::RegisterBlock = 0x4000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vcmp::RegisterBlock {
        Self::PTR
    }
}
impl Deref for VCMP {
    type Target = vcmp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for VCMP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VCMP").finish()
    }
}
#[doc = "VCMP"]
pub mod vcmp;
#[doc = "ADC0"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc0::RegisterBlock = 0x4000_2000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC0").finish()
    }
}
#[doc = "ADC0"]
pub mod adc0;
#[doc = "LEUART0"]
pub struct LEUART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LEUART0 {}
impl LEUART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const leuart0::RegisterBlock = 0x4008_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const leuart0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for LEUART0 {
    type Target = leuart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for LEUART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEUART0").finish()
    }
}
#[doc = "LEUART0"]
pub mod leuart0;
#[doc = "PCNT0"]
pub struct PCNT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCNT0 {}
impl PCNT0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const pcnt0::RegisterBlock = 0x4008_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcnt0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PCNT0 {
    type Target = pcnt0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PCNT0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCNT0").finish()
    }
}
#[doc = "PCNT0"]
pub mod pcnt0;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2c0::RegisterBlock = 0x4000_a000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        Self::PTR
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2C0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0").finish()
    }
}
#[doc = "I2C0"]
pub mod i2c0;
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x4008_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "WDOG"]
pub struct WDOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDOG {}
impl WDOG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdog::RegisterBlock = 0x4008_8000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdog::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WDOG {
    type Target = wdog::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDOG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDOG").finish()
    }
}
#[doc = "WDOG"]
pub mod wdog;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "MSC"]
    pub MSC: MSC,
    #[doc = "EMU"]
    pub EMU: EMU,
    #[doc = "RMU"]
    pub RMU: RMU,
    #[doc = "CMU"]
    pub CMU: CMU,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "ACMP0"]
    pub ACMP0: ACMP0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "PRS"]
    pub PRS: PRS,
    #[doc = "IDAC0"]
    pub IDAC0: IDAC0,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "VCMP"]
    pub VCMP: VCMP,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "LEUART0"]
    pub LEUART0: LEUART0,
    #[doc = "PCNT0"]
    pub PCNT0: PCNT0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "WDOG"]
    pub WDOG: WDOG,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            AES: AES {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            MSC: MSC {
                _marker: PhantomData,
            },
            EMU: EMU {
                _marker: PhantomData,
            },
            RMU: RMU {
                _marker: PhantomData,
            },
            CMU: CMU {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            ACMP0: ACMP0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            PRS: PRS {
                _marker: PhantomData,
            },
            IDAC0: IDAC0 {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            VCMP: VCMP {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            LEUART0: LEUART0 {
                _marker: PhantomData,
            },
            PCNT0: PCNT0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            WDOG: WDOG {
                _marker: PhantomData,
            },
        }
    }
}
