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


#[acmd_script( agent = "mewtwo", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn game_specialhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
	}

	
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
pub fn install() {
smashline::install_acmd_scripts!(
   game_specialhi );
}