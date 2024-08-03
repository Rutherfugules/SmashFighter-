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
 mod game_AttackHI3;
 mod game_AttackLW3;
 mod game_AttackS3;
 mod game_SpecialLW;
 mod game_SpecialLWEnd;
 mod game_SpecialAirLWEnd;
 mod game_SpecialAirLW;
 mod game_AttackS4;
 mod game_AttackHI4;
 mod game_AttackLW4;
 mod game_AttackAirF;
 mod game_AttackAirN;
 mod game_AttackDash;
 mod game_SpecialAirNTurn;
 mod game_SpecialAirN;
 mod game_SpecialN;
 mod game_SpecialNTurn;
 
  pub fn install() {
     game_Attack11::install();
	 game_AttackHI3::install();
	 game_AttackLW3::install();
	 game_AttackS3::install();
	 game_SpecialLW::install();
	 game_SpecialLWEnd::install();
     game_SpecialAirLWEnd::install();
	 game_SpecialAirLW::install();
	 game_AttackHI4::install();
	 game_AttackLW4::install();
	 game_AttackS4::install();
	 game_AttackAirF::install();
	 game_AttackAirN::install();
	 game_AttackDash::install();
	 game_SpecialAirNTurn::install();
	 game_SpecialAirN::install();
	 game_SpecialN::install();
	 game_SpecialN::install();
	 	 
 }
 



