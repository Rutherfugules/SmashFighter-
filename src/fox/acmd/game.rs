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
 mod game_AttackAirF;
 mod game_AttackAirLw;
 mod game_SpecialHi;
 mod game_SpecialHiHold;
 mod game_SpecialHiHoldAir;
 mod game_SpecialAirSEnd;
 mod game_SpecialLwStart;
 mod game_SpecialAirLwStart;
 mod game_MoveGround;
 mod game_MoveAir;
 
	
 
  pub fn install() {
     
	 game_Attack11::install();
	 game_AttackLw3::install();
	 game_AttackS3::install();
	 game_AttackAirF::install();
	 game_AttackAirLw::install();
	 game_SpecialHi::install();
	 game_SpecialHiHold::install();
	 game_SpecialHiHoldAir::install();
	 game_SpecialAirSEnd::install();
	 game_SpecialLwStart::install();
	 game_SpecialAirLwStart::install();
	 game_MoveGround::install();
	 game_MoveAir::install();

	 	 
 }
 



