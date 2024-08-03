use {
    smash::{
        lua2cpp::{L2CAgentBase,L2CFighterCommon,L2CFighterBase, *},
        phx::{Hash40,Vector3f,Vector2f},
        hash40,
        app::{lua_bind::*, sv_animcmd::*,*},
        lib::{lua_const::*,L2CValue}
    },
    smash_script::*,
    smashline::*
};

#[fighter_frame_callback]
pub fn landing_lag(fighter : &mut L2CFighterCommon) {  
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        if (status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR) && frame >5.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[fighter_frame_callback]
pub fn airdodge_lag(fighter : &mut L2CFighterCommon) {  
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        if (status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR) && frame >20.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[fighter_frame_callback]
pub fn airdodge_landinglag(fighter : &mut L2CFighterCommon) {  
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);

        if (status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE) && frame >2.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}
/*#[fighter_frame_callback]
pub fn universal_weight(fighter : &mut L2CFighterCommon) {
	unsafe {
		let boma = fighter.module_accessor;
		if status_kind == *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME {
			
		}
	}
}*/
pub fn install() {
    smashline::install_agent_frame_callbacks!(
        landing_lag,
		airdodge_lag,
		airdodge_landinglag
    );
    }
	
	//FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME