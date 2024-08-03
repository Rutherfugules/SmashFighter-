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

// Use this for general per-frame fighter-level hooks
#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_jab(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {//On hit...
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) !=0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) !=0{// if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
				
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_jab_shield(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {//On hit...
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) !=0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0{
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_dtilt(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {//On hit...
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{// if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
				
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_dtilt_shield(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {//On hit...
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_ftilt(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {//On hit...
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{ //if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
				
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_downspecial(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {//On hit...
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0  || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{// if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK,false);// ...Enable cancel
				
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_downspecial_shield(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {//On hit...// if input is side special... 1 // ...Once hitlag frames reach 1 (the last frame of hitlag)...
				StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END,false);// ...Enable c
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_fsmash(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {//On hit...
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 != 0) ||
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0  || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{ //if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
				
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_fsmash_shield(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {//On hit...
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 ||
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_upsmash(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {//On hit...
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0  || 
				(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) !=0{ //if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
				
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_upsmash_shield(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {//On hit...
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0  || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0{
						CancelModule::enable_cancel(fighter.module_accessor);
					
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
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
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0
					|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0{// if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
				
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn aerials_shields(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {//On hit...
				if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON) !=0 || 
					(cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) !=0 || (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) !=0
					|| (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) !=0{// if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
					
				}
			}			
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
pub fn ganon_dashattack(fighter : &mut L2CFighterCommon) {
	unsafe {	
		let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
		let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
		if [*FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {

			//On hit, change buffer window to current hitlag frames, so that at any poing of hitlag when an input is pressed it'll transition
			if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) { 
				ControlModule::set_command_life_extend(fighter.module_accessor, SlowModule::frame(fighter.module_accessor, *FIGHTER_SLOW_KIND_HIT) as u8); 
			}
			if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {//On hit...
			if (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0{ //if input is side special...// ...Once hitlag frames reach 1 (the last frame of hitlag)...
						CancelModule::enable_cancel(fighter.module_accessor);// ...Enable cancel
				
				}
			}			
		}
	}
}

pub fn install() {
    smashline::install_agent_frames!(
		ganon_jab,
		ganon_jab_shield,
		ganon_dtilt,
		ganon_dtilt_shield,
		ganon_ftilt,
		ganon_downspecial,
		ganon_downspecial_shield,
		ganon_fsmash,
		ganon_fsmash_shield,
		ganon_upsmash,
		ganon_upsmash_shield,
		ganon_dashattack,
		aerials,
		aerials_shields
    );
	acmd::install();
}
