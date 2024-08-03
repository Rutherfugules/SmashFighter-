#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod mario;
mod falco;
mod captain;
mod all;
mod ganon;
mod krool;
mod samus;
mod cloud;
mod mewtwo;
mod fox;
mod lucina;


#[skyline::main(name = "smashline_test")]
pub fn main() {
    //mario::install();
    //falco::install();
    captain::install();
	all::install();
	ganon::install();
	krool::install();
	samus::install();
	cloud::install();
	mewtwo::install();
	fox::install();
	lucina::install();
}