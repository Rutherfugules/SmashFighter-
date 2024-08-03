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

#[acmd_script( agent = "lucina", script = "game_specialairnendhi", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairnendhi(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 1.5, y: 1.5, z: 0.0});
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.5, 361, 90, 0, 45, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 8.5, 361, 90, 0, 45, 3.0, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 361, 90, 0, 45, 0.9, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.5, 361, 90, 0, 45, 0.9, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.5, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 4.5, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 4.5, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 4.5, false);
		AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.5, 361, 90, 0, 45, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 8.5, 361, 90, 0, 45, 3.0, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.5, 361, 90, 0, 45, 0.9, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.5, 361, 90, 0, 45, 0.9, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_HEAD, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 4.5, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 4.5, false);
		AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 4.5, false);
			AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
		macros::FT_MOTION_RATE(fighter, 0.8);
        AttackModule::clear_all(fighter.module_accessor);
    }
}
pub fn install() {
    smashline::install_acmd_scripts!(
        game_specialairnendhi);
}