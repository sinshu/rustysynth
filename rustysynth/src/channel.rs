#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
enum DataType {
    None,
    Rpn,
    Nrpn,
}

#[derive(Debug)]
#[non_exhaustive]
pub(crate) struct Channel {
    pub(crate) is_percussion_channel: bool,

    bank_number: u8,
    patch_number: u8,

    modulation: u16,
    volume: u16,
    pan: u16,
    expression: u16,
    hold_pedal: bool,

    reverb_send: u8,
    chorus_send: u8,

    rpn: u16,
    pitch_bend_range: u16,
    coarse_tune: i16,
    fine_tune: u16,

    pitch_bend: f32,

    last_data_type: DataType,
}

impl Channel {
    pub(crate) fn new(is_percussion_channel: bool) -> Self {
        let mut channel = Self {
            is_percussion_channel,
            bank_number: 0,
            patch_number: 0,
            modulation: 0,
            volume: 0,
            pan: 0,
            expression: 0,
            hold_pedal: false,
            reverb_send: 0,
            chorus_send: 0,
            rpn: 0,
            pitch_bend_range: 0,
            coarse_tune: 0,
            fine_tune: 0,
            pitch_bend: 0_f32,
            last_data_type: DataType::None,
        };

        channel.reset();

        channel
    }

    pub(crate) fn reset(&mut self) {
        self.bank_number = if self.is_percussion_channel { 128 } else { 0 };
        self.patch_number = 0;

        self.modulation = 0;
        self.volume = 100 << 7;
        self.pan = 64 << 7;
        self.expression = 127 << 7;
        self.hold_pedal = false;

        self.reverb_send = 40;
        self.chorus_send = 0;

        self.rpn = 0xFFFF;
        self.pitch_bend_range = 2 << 7;
        self.coarse_tune = 0;
        self.fine_tune = 8192;

        self.pitch_bend = 0_f32;
    }

    pub(crate) fn reset_all_controllers(&mut self) {
        self.modulation = 0;
        self.expression = 127 << 7;
        self.hold_pedal = false;

        self.rpn = 0xFFFF;

        self.pitch_bend = 0_f32;
    }

    pub(crate) fn set_bank(&mut self, value: u8) {
        self.bank_number = value;

        if self.is_percussion_channel {
            self.bank_number += 128;
        }
    }

    pub(crate) fn set_patch(&mut self, value: u8) {
        self.patch_number = value;
    }

    pub(crate) fn set_modulation_coarse(&mut self, value: u8) {
        self.modulation = (self.modulation & 0x7F) | ((value as u16) << 7);
    }

    pub(crate) fn set_modulation_fine(&mut self, value: u8) {
        self.modulation = (self.modulation & 0xFF80) | value as u16;
    }

    pub(crate) fn set_volume_coarse(&mut self, value: u8) {
        self.volume = (self.volume & 0x7F) | ((value as u16) << 7);
    }

    pub(crate) fn set_volume_fine(&mut self, value: u8) {
        self.volume = (self.volume & 0xFF80) | value as u16;
    }

    pub(crate) fn set_pan_coarse(&mut self, value: u8) {
        self.pan = (self.pan & 0x7F) | ((value as u16) << 7);
    }

    pub(crate) fn set_pan_fine(&mut self, value: u8) {
        self.pan = (self.pan & 0xFF80) | value as u16;
    }

    pub(crate) fn set_expression_coarse(&mut self, value: u8) {
        self.expression = (self.expression & 0x7F) | ((value as u16) << 7);
    }

    pub(crate) fn set_expression_fine(&mut self, value: u8) {
        self.expression = ((self.expression) & 0xFF80) | value as u16;
    }

    pub(crate) fn set_hold_pedal(&mut self, value: u8) {
        self.hold_pedal = value >= 64;
    }

    pub(crate) fn set_reverb_send(&mut self, value: u8) {
        self.reverb_send = value;
    }

    pub(crate) fn set_chorus_send(&mut self, value: u8) {
        self.chorus_send = value;
    }

    pub(crate) fn set_rpn_coarse(&mut self, value: u8) {
        self.rpn = (self.rpn & 0x7F) | ((value as u16) << 7);
        self.last_data_type = DataType::Rpn;
    }

    pub(crate) fn set_rpn_fine(&mut self, value: u8) {
        self.rpn = (self.rpn & 0xFF80) | value as u16;
        self.last_data_type = DataType::Rpn;
    }

    pub(crate) fn set_nrpn_coarse(&mut self, _value: u8) {
        self.last_data_type = DataType::Nrpn;
    }

    pub(crate) fn set_nrpn_fine(&mut self, _value: u8) {
        self.last_data_type = DataType::Nrpn;
    }

    pub(crate) fn data_entry_coarse(&mut self, value: u8) {
        if self.last_data_type != DataType::Rpn {
            return;
        }

        if self.rpn == 0 {
            self.pitch_bend_range = (self.pitch_bend_range & 0x7F) | ((value as u16) << 7);
        } else if self.rpn == 1 {
            self.fine_tune = (self.fine_tune & 0x7F) | ((value as u16) << 7);
        } else if self.rpn == 2 {
            self.coarse_tune = (value - 64) as i16;
        }
    }

    pub(crate) fn data_entry_fine(&mut self, value: u8) {
        if self.last_data_type != DataType::Rpn {
            return;
        }

        if self.rpn == 0 {
            self.pitch_bend_range = (self.pitch_bend_range & 0xFF80) | value as u16;
        } else if self.rpn == 1 {
            self.fine_tune = (self.fine_tune & 0xFF80) | value as u16;
        }
    }

    pub(crate) fn set_pitch_bend(&mut self, value1: u8, value2: u8) {
        self.pitch_bend =
            (1_f32 / 8192_f32) * (((value1 as i32) | ((value2 as i32) << 7)) - 8192) as f32;
    }

    pub(crate) fn get_bank_number(&self) -> u8 {
        self.bank_number
    }

    pub(crate) fn get_patch_number(&self) -> u8 {
        self.patch_number
    }

    pub(crate) fn get_modulation(&self) -> f32 {
        (50_f32 / 16383_f32) * self.modulation as f32
    }

    pub(crate) fn get_volume(&self) -> f32 {
        (1_f32 / 16383_f32) * self.volume as f32
    }

    pub(crate) fn get_pan(&self) -> f32 {
        (100_f32 / 16383_f32) * self.pan as f32 - 50_f32
    }

    pub(crate) fn get_expression(&self) -> f32 {
        (1_f32 / 16383_f32) * self.expression as f32
    }

    pub(crate) fn get_hold_pedal(&self) -> bool {
        self.hold_pedal
    }

    pub(crate) fn get_reverb_send(&self) -> f32 {
        (1_f32 / 127_f32) * self.reverb_send as f32
    }

    pub(crate) fn get_chorus_send(&self) -> f32 {
        (1_f32 / 127_f32) * self.chorus_send as f32
    }

    pub(crate) fn get_pitch_bend_range(&self) -> f32 {
        (self.pitch_bend_range >> 7) as f32 + 0.01_f32 * (self.pitch_bend_range & 0x7F) as f32
    }

    pub(crate) fn get_tune(&self) -> f32 {
        self.coarse_tune as f32 + (1_f32 / 8192_f32) * (self.fine_tune - 8192) as f32
    }

    pub(crate) fn get_pitch_bend(&self) -> f32 {
        self.get_pitch_bend_range() * self.pitch_bend
    }
}
