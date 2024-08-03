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

#[acmd_script( agent = "mewtwo", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn game_specials(fighter: &mut L2CAgentBase) {
let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.2);
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 7.4, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 10.2, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_G);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.0, 280, 16, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, 0, 1.15, 280, 16, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, true, true);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(fighter.lua_state_agent, 20.0);
    for _ in 0..7 {
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_TARGET_OBJECT_ID);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, Hash40::new("throw"), target, target_group, target_no);
    }
}
frame(fighter.lua_state_agent, 36.0);
if macros::is_excute(fighter) {
    shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_GRAVITY_NORMAL);
}
frame(fighter.lua_state_agent, 40.0);
if macros::is_excute(fighter) {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_HIT);
}
}    
pub fn install() {
smashline::install_acmd_scripts!(
    game_specials );
}
