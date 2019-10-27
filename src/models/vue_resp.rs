use serde::Serialize;
#[derive(Debug, Serialize, new)]
pub struct ResponseMsg<T> {
	pub code:i32,
	pub data:T,
}

impl ResponseMsg<T> {
	pub fn new_ok(t: T) -> Self {
		Self{
			code:20000_i32,
			data:t,
		}
	}
}
