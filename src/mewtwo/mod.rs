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

pub mod acmd;

#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_jab(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {

			
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0 {
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}



#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_dtilt(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {

			
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0 
					|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) !=0 {
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}


#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_ftilt(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) {

			
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) !=0 {
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}



#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_upspecial(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
				if  (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0{
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_100,false);
				}
				if  (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
				}
			}			
		}
	}
	
	#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_uptilt(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_HI3].contains(&status_kind) {

			
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0{
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}

	
	#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_upsmash(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {

			
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) !=0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0{
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}

	#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_dsmash(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {

			
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) !=0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) !=0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) !=0{
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}
	
	#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn aerials(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {//On hit...
				if ((cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0
					|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0){// if input is side special...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
					
				}
			}			
		}
	}
}



pub fn install() {
    smashline::install_agent_frames!(
		mewtwo_jab,
		mewtwo_dtilt,
		mewtwo_ftilt,
		mewtwo_uptilt,
		mewtwo_upsmash,
		mewtwo_upspecial,
		mewtwo_dsmash,
		aerials,
		
		
    );
	acmd::install();
}