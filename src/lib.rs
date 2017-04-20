pub extern crate libsrl;
use libsrl::db::Database;
use libsrl::cell::Cell;

pub enum StopReason { Win, Fail, Timeout }

pub trait Botfather {
	fn call(&self, db : &mut Database, target : &Cell);
	fn assess(&mut self, stop_reason : StopReason, milliseconds : u32);
}
