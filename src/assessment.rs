pub enum Stop { Timeout, Win, Lose }

pub struct Assessment {
	pub stop : Stop,
	pub time : i64
}
