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

#[acmd_script( agent = "samus", script = "game_specialnfiremax", category = ACMD_GAME, low_priority )]
unsafe fn game_specialnfiremax(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_CSHOT, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
	
	 frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
      macros::FT_MOTION_RATE(fighter, 0.2);
    }
	
}    
    
pub fn install() {
    smashline::install_acmd_scripts!(
     game_specialnfiremax);
 }
