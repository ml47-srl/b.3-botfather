pub extern crate libsrl;
use libsrl::db::Database;
use libsrl::cell::Cell;

pub enum StopReason { Win, Fail, Timeout }

pub trait Botfather : Clone + Send + 'static {
	fn call(&self, db: &mut Database, target: &Cell);
	fn assess(&mut self, stop_reason: StopReason, milliseconds: u32);

	fn to_string(&self) -> String;
	fn by_string(string: &str) -> Self;
	fn gen() -> Self;
}
