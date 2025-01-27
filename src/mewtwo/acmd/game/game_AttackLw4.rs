use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40,Vector3f,Vector2f},
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::lua_const::*,
    },
    smash_script::*,
    smashline::* 
};


#[acmd_script( agent = "mewtwo", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn game_attacklw4(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

	frame(fighter.lua_state_agent, 1.0);
	macros::FT_MOTION_RATE(fighter, 2.5);

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
		macros::FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 21.0);
	macros::FT_MOTION_RATE(fighter, 0.2);
    if macros::is_excute(fighter) { 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 280, 150, 50, 0, 14.0, 0.0, 4.0, 13.7, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 20.0, 280, 150, 50, 0, 14.0, 0.0, 4.0, 13.7, None, None, None, 2.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
		macros::FT_MOTION_RATE(fighter, 0.1);
        AttackModule::clear_all(fighter.module_accessor);
    }
}
pub fn install() {
smashline::install_acmd_scripts!(
   game_attacklw4 );
}