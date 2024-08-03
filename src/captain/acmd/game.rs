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

 mod game_AttackLw3;
 mod game_Attack100End;
 mod game_SpecialSEnd;
 mod game_SpecialLw;
 mod game_SpecialAirLw;
 mod game_AttackS4;
 mod game_AttackAirN;
 mod game_AttackAirHI;
 mod game_AttackS3;
 mod game_AttackAirF;
 mod game_AttackDash;
 mod game_AttackAirLW;
 mod game_SpecialN; 
 mod game_AttackHi3;
 mod game_AttackHi4;
 mod game_AttackLw4;
 mod game_SpecialNTurn; 
 mod game_SpecialAirNTurn; 
 mod game_SpecialAirN; 
 mod game_SpecialAirSEnd; 
 
  pub fn install() {
	  game_Attack100End::install();
     game_AttackLw3::install();
     game_SpecialSEnd::install();
	 game_SpecialLw::install();
	 game_SpecialAirLw::install();
	 game_AttackS4::install();
	 game_AttackAirN::install();
	 game_AttackAirHI::install();
	 game_AttackS3::install();
	 game_AttackAirF::install();
	 game_AttackDash::install();
	 game_AttackAirLW::install();
	 game_SpecialN::install();	 
	 game_AttackHi3::install();
	 game_AttackHi4::install();
	 game_AttackLw4::install();
	 game_SpecialNTurn::install();
	 game_SpecialAirNTurn::install();
	 game_SpecialAirN::install();
	 game_SpecialAirSEnd::install();
	 	 
 }
 



