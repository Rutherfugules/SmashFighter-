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

	mod effect_SpecialN;

 
	
 
  pub fn install() {
     
	effect_SpecialN::install();

	 	 
 }
 


