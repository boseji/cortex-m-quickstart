#![deny(warnings)]
#![deny(unused_variables)]
#![deny(dead_code)]
#![deny(non_snake_case)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_semihosting;
//#[macro_use(exception, interrupt)]
#[macro_use(exception)]
extern crate stm32f103xx;

use stm32f103xx::GPIOC;
use stm32f103xx::GPIOB;
use stm32f103xx::RCC;
use stm32f103xx::SYST;
use cortex_m::peripheral::SystClkSource;
use cortex_m::asm::{nop};
use cortex_m::interrupt::{enable};

fn main() {
    // RCC IOPORT C Enable
    // Clock enabled for GPIOC
    unsafe {
        (*RCC.get()).apb2enr.modify(|_, w| w.iopcen().enabled());
    }

    // RCC IOPORT B Enable
    // Clock enabled for GPIOB
    unsafe {
        (*RCC.get()).apb2enr.modify(|_, w| w.iopben().enabled());
    }

    // SysTick Timer Configuration
    unsafe {
        (*SYST.get()).set_clock_source(SystClkSource::Core);
        (*SYST.get()).set_reload(8_000_000); // 1s
        (*SYST.get()).enable_counter();
        (*SYST.get()).enable_interrupt();
    }

    // BIT Set Reset Register GPIOC - BitSet for Bit13
    // Output register is Set as HIGH
    unsafe {
        (*GPIOC.get()).bsrr.write(|w| w.bs13().set());
    }

    // BIT Set Reset Register GPIOB - BitSet for Bit12
    // Output register is Set as HIGH
    unsafe {
        (*GPIOB.get()).bsrr.write(|w| w.bs12().set());
    }

    // GPIO MODER register & CNF register GPIOC - BIT 13 Set 
    // Configure port as PUSH-PULL HIGH speed OUTPUT port
    unsafe {
        (*GPIOC.get()).crh.modify(|_, w| w.mode13().output().cnf13().push());
    }

    // GPIO MODER register & CNF register GPIOB - BIT 12 Set 
    // Configure port as PUSH-PULL HIGH speed OUTPUT port
    unsafe {
        (*GPIOB.get()).crh.modify(|_, w| w.mode12().output().cnf12().push());
    }

    // BIT Set Reset Register GPIOC - BitReset for Bit13
    // Output register is Set as LOW and Active Low LED lights up
    unsafe {
        (*GPIOC.get()).bsrr.write(|w| w.br13().reset());
    }   

    // BIT Set Reset Register GPIOB - BitReset for Bit12
    // Output register is Set as LOW and Active Low LED lights up
    unsafe {
        (*GPIOB.get()).bsrr.write(|w| w.br12().reset());
    } 

    // Enable the Interrupts
    unsafe {
        enable();
    }

    loop {
        delay();
        //nop();
        unsafe {
            (*GPIOC.get()).bsrr.write(|w| w.bs13().set());
        }
        delay();
        //nop();
        unsafe {
            (*GPIOC.get()).bsrr.write(|w| w.br13().reset());
        }   
    }
}

// Blocking Delay Loop generating 1 Second delay
fn delay() {
    for _x in 0..13_125 {
        nop();
    }
}

// Attach the SYS_TICK Interrupt to the tick() function
exception!(SYS_TICK, tick);

// SysTick Timer Interrupt Function
fn tick() {
    // Toggle the GPIOB Pin 12
    unsafe {
        (*GPIOB.get()).odr.modify(|r, w| {
                // Check if the Pin is Set
                if r.odr12().bit_is_set() {
                    // Clear if its set
                    w.odr12().clear_bit();
                } else {
                    // Set the bit if its not
                    w.odr12().set_bit();
                }
                // Return the value
                w
        });
    } 

}