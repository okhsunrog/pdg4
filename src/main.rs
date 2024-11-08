#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    gpio::{Level, Output, Speed},
    time::{khz, mhz},
};
use embassy_time::{Duration, Timer};
#[allow(unused_imports)]
use panic_probe as _;
use rtt_target::{rtt_init, ChannelMode::*};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_stm32::Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: mhz(8),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL85,
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV2),
            divr: Some(PllRDiv::DIV2),
        });

        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV1;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
    }
    let p = embassy_stm32::init(config);
    let channels = rtt_init! {
        up: {
            0: {
                size: 512,
                mode: BlockIfFull,
            }
            1: {
                size: 512,
                mode: BlockIfFull,
            }
            2: {
                size: 2200,
                mode: BlockIfFull,
            }
        }
        down: {
            0: {
                size: 512,
                mode: BlockIfFull,
            }
        }
    };
    rtt_target::set_defmt_channel(channels.up.0);
    rtt_target::set_print_channel(channels.up.1);

    info!("Hello from STM32G4 board!");
    spawner.must_spawn(system_ticker());
    loop {}
}

#[embassy_executor::task]
async fn system_ticker() {
    loop {
        info!("Tick!");
        Timer::after(Duration::from_millis(2000)).await;
    }
}
