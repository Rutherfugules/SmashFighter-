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
 mod game_Attack12;
 mod game_AttackHi3;
 mod game_AttackLw3;
 mod game_AttackS3;
 mod game_AttackS4;
 mod game_AttackHi4;
 mod game_AttackLw4;
 mod game_AttackDash;
 mod game_AttackAirN;
 mod game_SpecialS1;
 mod game_SpecialHi;
 mod game_SpecialS4Hi;
 mod game_SpecialS4S;
 mod game_SpecialS4Lw;
 mod game_SpecialAirS1;
 mod game_SpecialAirNEndMax;
 mod game_SpecialAirNEnd;
 mod game_SpecialAirNEndHi;
 mod game_SpecialAirNEndLw;
 mod game_SpecialAirS2Hi;
 mod game_SpecialAirS2Lw;
 mod game_SpecialAirS3Hi;
 mod game_SpecialAirS3Lw;
 mod game_SpecialAirS3S;
 mod game_SpecialAirS4Hi;
 mod game_SpecialAirS4Lw;
 mod game_SpecialAirS4S;
 

 




 
	
 
  pub fn install() {
     
	 game_Attack11::install();
	 game_Attack12::install();
	 game_AttackHi3::install();
	 game_AttackS3::install();
	 game_AttackLw3::install();
	 game_AttackS4::install();
	 game_AttackHi4::install();
	 game_AttackLw4::install();
	 game_AttackDash::install();
	 game_AttackAirN::install();
	 game_SpecialS1::install();
	 game_SpecialHi::install();
	 game_SpecialS4Hi::install();
	 game_SpecialS4S::install();
	 game_SpecialS4Lw::install();
	 game_SpecialAirNEnd::install();
	 game_SpecialAirNEndHi::install();
	 game_SpecialAirNEndLw::install();
	 game_SpecialAirNEndMax::install();
	 game_SpecialAirS1::install();
	 game_SpecialAirS2Hi::install();
	 game_SpecialAirS2Lw::install();
	 game_SpecialAirS3Hi::install();
	 game_SpecialAirS3Lw::install();
	 game_SpecialAirS3S::install();
	 game_SpecialAirS4Hi::install();
	 game_SpecialAirS4Lw::install();
	 game_SpecialAirS4S::install();
	


	 	 
 }
 



