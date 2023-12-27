use modular_bitfield::bitfield;
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct Nr0 {
    shift: B3,
    sweep_direction: B1,
    period: B3,
    empty: B1,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct Nr3 {
    frequency: B8,
}

#[bitfield]
#[derive(Debug, Clone, Copy)]
pub struct Nr4 {
    period: B3,
    empty: B3,
    length_enable: B1,
    trigger: B1,
}

struct FrequencySweep {
    nr0: Nr0,
    nr3: Nr3,
    nr4: Nr4,
    freq: i32,
    shadow_freq: i32,
    enabled: bool,
    timer: i32,
    overflow: bool,
    period_load: u8,
}

impl FrequencySweep {
    fn power_off(&mut self) {
        self.enabled = false;
        self.overflow = false;

        self.timer = 0;
        self.freq = 0;
        self.shadow_freq = 0;

        self.nr0.bytes = [0];
    }

    fn trigger(&mut self) {
        let period = self.nr0.period();
        self.overflow = false;
        self.shadow_freq = self.freq;

        self.timer = if period != 0 { period as i32 } else { 8 };

        self.enabled = period != 0 || self.nr0.shift() != 0;

        if self.nr0.shift() > 0 {
            self.calc_freq();
        }
    }

    fn calc_freq(&mut self) -> i32 {
        let mut new_frequency = self.shadow_freq >> self.nr0.shift();

        let is_decreasing = self.nr0.sweep_direction() == 1;
        if is_decreasing {
            new_frequency = self.shadow_freq - new_frequency;
        } else {
            new_frequency = self.shadow_freq + new_frequency;
        }

        if new_frequency > 2047 {
            self.overflow = true;
        }

        return new_frequency;
    }

    fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        if self.timer <= 0 {
            self.timer = if self.nr0.period() != 0 {
                self.nr0.period() as i32
            } else {
                8
            };

            if self.nr0.period() != 0 {
                let new_freq = self.calc_freq();

                if !self.overflow && self.nr0.shift() != 0 {
                    self.shadow_freq = new_freq;
                    self.freq = new_freq;
                    self.calc_freq();
                }
            }
        }
    }
}
