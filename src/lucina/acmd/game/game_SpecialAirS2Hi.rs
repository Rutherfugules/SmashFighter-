use {
    smash::{
        lua2cpp::{L2CAgentBase, L2CFighterCommon},
        phx::{Hash40,Vector3f,Vector2f},
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*,
        hash40
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "lucina", script = "game_specialairs2hi", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairs2hi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.85, 363, 30, 0, 30, 4.5, 0.0, 6.0, 12.5, Some(0.0), Some(16.0), Some(12.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.85, 363, 30, 0, 30, 3.0, 0.0, 6.0, 16.0, Some(0.0), Some(18.0), Some(16.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.85, 363, 30, 0, 30, 4.5, 0.0, 9.0, 8.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
        game_specialairs2hi);
}