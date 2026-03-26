pub struct Equipment {
    pub name: &'static str,
    pub alias: &'static [&'static str],
    pub slot: &'static str,
    pub level: u32,
    pub count: u32,
}

pub const EQUIPMENTS: [Equipment; 2] = [RUTABIS_HAT, ABSOLABS_HAT];

const RUTABIS_HAT: Equipment = Equipment {
    name: "루타비스 모자",
    alias: &["하이네스"],
    slot: "armor",
    level: 150,
    count: 12,
};

const ABSOLABS_HAT: Equipment = Equipment {
    name: "앱솔랩스 모자",
    alias: &[],
    slot: "armor",
    level: 160,
    count: 12,
};

pub fn find_by_name_or_alias(query: &str) -> Option<&'static Equipment> {
    let normalized_query = query.trim().to_lowercase();
    if normalized_query.is_empty() {
        return None;
    }

    EQUIPMENTS.iter().find(|equipment| {
        equipment.name.to_lowercase() == normalized_query
            || equipment
                .alias
                .iter()
                .any(|alias| alias.to_lowercase() == normalized_query)
    })
}

pub fn options() -> Vec<&'static str> {
    let mut result = Vec::new();

    for equipment in EQUIPMENTS.iter().filter(|equipment| equipment.slot == "armor") {
        result.push(equipment.name);
        for alias in equipment.alias {
            result.push(alias);
        }
    }

    result
}
