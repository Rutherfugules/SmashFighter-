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

#[acmd_script( agent = "fox", script = "game_specialhihold", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhihold(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 0.6);
	
    frame(fighter.lua_state_agent, 20.0);
    for _ in 0..7 {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 105, 20, 0, 20, 8.0, 0.0, 8.0, 0.0, None, None, None, 0.8, 0.6, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 1.0);
}
}   
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialhihold);
}