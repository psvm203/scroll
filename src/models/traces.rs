pub const EQUIPMENT_SLOT_WEAPON: &str = "weapon";
pub const EQUIPMENT_SLOT_ARMOR: &str = "armor";
pub const EQUIPMENT_SLOT_GLOVE: &str = "glove";
pub const EQUIPMENT_SLOT_ACCESSORY: &str = "accessory";
pub const EQUIPMENT_SLOT_HEART: &str = "heart";

pub fn is_supported_equipment_slot(equipment_slot: &str) -> bool {
    matches!(
        equipment_slot,
        EQUIPMENT_SLOT_WEAPON
            | EQUIPMENT_SLOT_ARMOR
            | EQUIPMENT_SLOT_GLOVE
            | EQUIPMENT_SLOT_ACCESSORY
            | EQUIPMENT_SLOT_HEART
    )
}

pub fn calculate_total_trace_required(
    equipment_slot: &str,
    equipment_level: u32,
    upgradeable_count: u32,
    trace_probability: u32,
) -> Option<u32> {
    trace_required_per_slot(equipment_slot, equipment_level, trace_probability)
        .map(|per_slot| per_slot.saturating_mul(upgradeable_count))
}

fn trace_required_per_slot(
    equipment_slot: &str,
    equipment_level: u32,
    trace_probability: u32,
) -> Option<u32> {
    let level_bucket = trace_level_bucket(equipment_level)?;

    match equipment_slot {
        EQUIPMENT_SLOT_WEAPON => match trace_probability {
            100 => match level_bucket {
                100 => Some(13),
                110 => Some(16),
                120 => Some(57),
                130 => Some(72),
                140 => Some(90),
                150 => Some(185),
                160 => Some(220),
                200 => Some(435),
                250 => Some(850),
                _ => None,
            },
            70 => match level_bucket {
                100 => Some(17),
                110 => Some(20),
                120 => Some(72),
                130 => Some(93),
                140 => Some(117),
                150 => Some(240),
                160 => Some(285),
                200 => Some(565),
                250 => Some(1100),
                _ => None,
            },
            30 => match level_bucket {
                100 => Some(20),
                110 => Some(24),
                120 => Some(87),
                130 => Some(114),
                140 => Some(138),
                150 => Some(290),
                160 => Some(345),
                200 => Some(675),
                250 => Some(1325),
                _ => None,
            },
            15 => match level_bucket {
                100 => Some(24),
                110 => Some(29),
                120 => Some(104),
                130 => Some(133),
                140 => Some(166),
                150 => Some(342),
                160 => Some(414),
                200 => Some(910),
                250 => Some(1560),
                _ => None,
            },
            _ => None,
        },
        EQUIPMENT_SLOT_ARMOR => match trace_probability {
            100 => match level_bucket {
                100 => Some(13),
                110 => Some(16),
                120 => Some(57),
                130 => Some(72),
                140 => Some(90),
                150 => Some(185),
                160 => Some(220),
                200 => Some(435),
                250 => Some(850),
                _ => None,
            },
            70 => match level_bucket {
                100 => Some(17),
                110 => Some(20),
                120 => Some(72),
                130 => Some(93),
                140 => Some(117),
                150 => Some(240),
                160 => Some(285),
                200 => Some(565),
                250 => Some(1100),
                _ => None,
            },
            30 => match level_bucket {
                100 => Some(20),
                110 => Some(24),
                120 => Some(87),
                130 => Some(114),
                140 => Some(138),
                150 => Some(290),
                160 => Some(345),
                200 => Some(675),
                250 => Some(1325),
                _ => None,
            },
            15 => match level_bucket {
                100 => Some(24),
                110 => Some(29),
                120 => Some(104),
                130 => Some(133),
                140 => Some(166),
                150 => Some(342),
                160 => Some(414),
                200 => Some(910),
                250 => Some(1560),
                _ => None,
            },
            _ => None,
        },
        EQUIPMENT_SLOT_GLOVE => match trace_probability {
            100 => match level_bucket {
                100 => Some(17),
                110 => Some(20),
                120 => Some(75),
                130 => Some(96),
                140 => Some(120),
                150 => Some(245),
                160 => Some(295),
                200 => Some(580),
                250 => Some(1135),
                _ => None,
            },
            70 => match level_bucket {
                100 => Some(23),
                110 => Some(27),
                120 => Some(96),
                130 => Some(123),
                140 => Some(156),
                150 => Some(320),
                160 => Some(385),
                200 => Some(750),
                250 => Some(1475),
                _ => None,
            },
            30 => match level_bucket {
                100 => Some(27),
                110 => Some(33),
                120 => Some(117),
                130 => Some(150),
                140 => Some(186),
                150 => Some(380),
                160 => Some(460),
                200 => Some(900),
                250 => Some(1770),
                _ => None,
            },
            15 => match level_bucket {
                100 => Some(32),
                110 => Some(38),
                120 => Some(139),
                130 => Some(178),
                140 => Some(221),
                150 => Some(456),
                160 => Some(550),
                200 => Some(1080),
                250 => Some(2080),
                _ => None,
            },
            _ => None,
        },
        EQUIPMENT_SLOT_ACCESSORY => match trace_probability {
            100 => match level_bucket {
                100 => Some(18),
                110 => Some(22),
                120 => Some(57),
                130 => Some(100),
                140 => Some(125),
                160 => Some(185),
                200 => Some(360),
                _ => None,
            },
            70 => match level_bucket {
                100 => Some(24),
                110 => Some(28),
                120 => Some(72),
                130 => Some(130),
                140 => Some(160),
                160 => Some(240),
                200 => Some(470),
                _ => None,
            },
            30 => match level_bucket {
                100 => Some(28),
                110 => Some(34),
                120 => Some(87),
                130 => Some(155),
                140 => Some(195),
                160 => Some(285),
                200 => Some(560),
                _ => None,
            },
            _ => None,
        },
        EQUIPMENT_SLOT_HEART => match trace_probability {
            100 => match level_bucket {
                100 => Some(36),
                130 => Some(200),
                _ => None,
            },
            70 => match level_bucket {
                100 => Some(47),
                130 => Some(260),
                _ => None,
            },
            30 => match level_bucket {
                100 => Some(56),
                130 => Some(310),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

fn trace_level_bucket(level: u32) -> Option<u32> {
    if level < 100 {
        None
    } else if level < 110 {
        Some(100)
    } else if level < 120 {
        Some(110)
    } else if level < 130 {
        Some(120)
    } else if level < 140 {
        Some(130)
    } else if level < 150 {
        Some(140)
    } else if level < 160 {
        Some(150)
    } else if level < 200 {
        Some(160)
    } else if level < 250 {
        Some(200)
    } else {
        Some(250)
    }
}
