use collections::HashMap;

use network::ClientId;


pub type Components<T> = HashMap<ClientId, T>;


pub struct Player {
	pub missile_index: u64
}
