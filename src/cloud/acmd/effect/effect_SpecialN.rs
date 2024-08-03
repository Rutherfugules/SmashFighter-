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

#[acmd_script( agent = "cloud", script = "effect_specialn", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialn(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

	frame(fighter.lua_state_agent, 16.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_WORK(fighter, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 7, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
	}
	
	frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("cloud_hakogeki_scrape"), Hash40::new("top"), -1.6, 0, -10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
	
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
	}
	
	frame(fighter.lua_state_agent, 21.0);
	if macros::is_excute(fighter) {
		macros::AFTER_IMAGE_OFF(fighter, 4);
	}
	
	frame(fighter.lua_state_agent, 25.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
		
	}
}



pub fn install() {
    smashline::install_acmd_scripts!(
     effect_specialn);
 }