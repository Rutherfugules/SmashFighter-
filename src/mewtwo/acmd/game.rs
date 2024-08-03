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
 mod game_Attack100End;
 mod game_AttackLw3;
 mod game_AttackAirN;
 mod game_AttackAirF;
 mod game_AttackHi3;
 mod game_AttackHi4;
 mod game_AttackS3;
 mod game_AttackLw4;
 mod game_SpecialHi;
 mod game_SpecialLw;
 mod game_SpecialAirLw;
 mod game_SpecialS;
 mod game_ThrowLw;
 mod game_ThrowF;

 
	
 
  pub fn install() {
     
	 game_Attack11::install();
	 game_Attack100End::install();
	 game_AttackLw3::install();
	 game_AttackAirN::install();
	 game_AttackAirF::install();
	 game_AttackHi3::install();
	 game_AttackHi4::install();
	 game_AttackS3::install();
	 game_SpecialHi::install();
	 game_SpecialLw::install();
	 game_SpecialAirLw::install();
	 game_SpecialS::install();
	 game_ThrowLw::install();
	 game_ThrowF::install();
	


	 	 
 }
 



