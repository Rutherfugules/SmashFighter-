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

 mod effect_AttackS4;
 mod effect_AttackS4Lw;
 mod effect_AttackS4Hi;
 
	
 
  pub fn install() {
     
	 effect_AttackS4::install();
	 effect_AttackS4Hi::install();
	 effect_AttackS4Lw::install();

	 	 
 }
 



