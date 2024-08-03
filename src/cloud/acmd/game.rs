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

 mod game_Attack11;
 mod game_AttackLw3;
 mod game_AttackS3;
 mod game_AttackHi4;
 mod game_AttackAirF;
 mod game_AttackAirB;
 mod game_SpecialS3_LB;
 mod game_SpecialAirS3_LB;
 mod game_SpecialS3;
 mod game_SpecialAirS3;
 mod game_SpecialLw;
 mod game_SpecialAirLw;
 mod game_SpecialN;
 
 
	
 
  pub fn install() {
     
	 game_Attack11::install();
	 game_AttackLw3::install();
	 game_AttackS3::install();
	 game_AttackHi4::install();
	 game_AttackAirF::install();
	 game_AttackAirB::install();
	 game_SpecialS3_LB::install();
	 game_SpecialAirS3_LB::install();
	 game_SpecialS3::install();
	 game_SpecialAirS3::install();
	 game_SpecialLw::install();
	 game_SpecialAirLw::install();
	 game_SpecialN::install();
		
	 	 
 }
 



