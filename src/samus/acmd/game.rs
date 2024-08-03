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
 mod game_AttackHi4;
 mod game_AttackLw3;
 mod game_AttackLw4;
 mod game_AttackS3;
 mod game_AttackS4;
 mod game_AttackS4Hi;
 mod game_AttackS4Lw;
 mod game_AttackDash;
 mod game_AttackAirF;
 mod game_AttackAirHi;
 mod game_AttackAirN;
 mod game_SpecialS;
 mod game_SpecialNFire;
 mod game_SpecialNFireMax;
 mod game_SpecialAirNFire;
 mod game_SpecialAirNFireMax;
 mod game_SpecialHi;
 mod game_SpecialAirHi;
 mod game_AirCatch;
 
	
 
  pub fn install() {
     
	 game_Attack11::install();
	 game_Attack12::install();
	 game_AttackHi3::install();
	 game_AttackHi4::install();
	 game_AttackLw3::install();
	 game_AttackLw4::install();
	 game_AttackS3::install();
	 game_AttackS4::install();
	 game_AttackS4Hi::install();
	 game_AttackS4Lw::install();
	 game_AttackDash::install();
	 game_AttackAirF::install();
	 game_AttackAirHi::install();
	 game_AttackAirN::install();
	 game_SpecialS::install();
	 game_SpecialNFire::install();
	 game_SpecialNFireMax::install();
	 game_SpecialAirNFire::install();
	 game_SpecialAirNFireMax::install();
	 game_SpecialHi::install();
	 game_SpecialAirHi::install();
	 game_AirCatch::install();

	 	 
 }
 



