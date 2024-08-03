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
 mod game_AttackDash;
 mod game_AttackHi3;
 mod game_AttackHi4;
 mod game_AttackS3;
 mod game_AttackS3Hi;
 mod game_AttackS3Lw;
 mod game_AttackLw3;
 mod game_AttackLw4;
 mod game_AttackAirN;
 mod game_AttackAirHi;
 mod game_AttackAirF;
 mod game_SpecialNFire;
 mod game_SpecialAirNFire;
 mod game_SpecialSThrow;
 mod game_SpecialAirSThrow;
 mod game_ThrowHi;
 mod game_ThrowLw;
 
	
 
  pub fn install() {
     
	 game_Attack11::install();
	 game_AttackDash::install();
	 game_AttackHi3::install();
	 game_AttackHi4::install();
	 game_AttackS3::install();
	 game_AttackS3Hi::install();
	 game_AttackS3Lw::install();
	 game_AttackLw3::install();
	 game_AttackLw4::install();
	 game_AttackAirN::install();
	 game_AttackAirHi::install();
	 game_AttackAirF::install();
	 game_SpecialNFire::install();
	 game_SpecialAirNFire::install();
	 game_SpecialSThrow::install();
	 game_SpecialAirSThrow::install();
	 game_ThrowHi::install();
	 game_ThrowLw::install();

	 	 
 }
 



