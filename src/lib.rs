pub extern crate libsrl;
use libsrl::db::Database;
use libsrl::cell::Cell;

mod assessment;
use assessment::Assessment;

pub trait Botfather {
	fn call(db : &mut Database, target : &Cell);
	fn assess(assessment : &Assessment);
}
