use modular_bitfield::prelude::*;
#[bitfield]
#[derive(Debug)]
pub struct JoypadInput {
    right_or_a: B1,
    left_or_b: B1,
    up_or_select: B1,
    down_or_start: B1,
    select_direction: B1,
    select_action: B1,
    empty: B2,
}
