use maplestory::prelude::*;

mod constants {
    pub const API_ORIGIN: &str = "https://nexon-open-api-proxy.psvm203.workers.dev";
    pub const ENHANCE_MASTERY: &str = "강화의 달인";
    pub const UPGRADE_SALVATION: &str = "실패를 두려워 않는";
}

pub struct ProbabilityContext {
    _world_name: String,
    pub handicraft: u32,
    pub enhance_mastery: u32,
    pub upgrade_salvation: u32,
}

pub async fn fetch_probability_context(
    character_name: String,
) -> Result<ProbabilityContext, ApiError> {
    let api = MaplestoryApi::builder().origin(constants::API_ORIGIN).build();
    let ocid = api.get_id(&character_name).await?.ocid;
    let handicraft = api.get_character_propensity(&ocid, None).await?.handicraft_level as u32;
    let character_basic = api.get_character_basic(&ocid, None).await?;
    let world_name = character_basic.world_name;

    if let Some(guild_name) = character_basic.character_guild_name {
        let oguild_id = api.get_guild_id(&guild_name, &world_name).await?.oguild_id;
        let guild_skills = api.get_guild_basic(&oguild_id, None).await?.guild_skill;

        let enhance_mastery = guild_skills
            .iter()
            .find(|x| x.skill_name == constants::ENHANCE_MASTERY)
            .map_or(0, |skill| skill.skill_level as u32);

        let upgrade_salvation = guild_skills
            .iter()
            .find(|x| x.skill_name == constants::UPGRADE_SALVATION)
            .map_or(0, |skill| skill.skill_level as u32);

        Ok(ProbabilityContext {
            _world_name: world_name,
            handicraft,
            enhance_mastery,
            upgrade_salvation,
        })
    } else {
        Ok(ProbabilityContext {
            _world_name: world_name,
            handicraft,
            enhance_mastery: 0,
            upgrade_salvation: 0,
        })
    }
}
