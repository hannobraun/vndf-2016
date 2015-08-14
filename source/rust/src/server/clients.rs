use std::collections::HashMap;
use std::net::SocketAddr;

use shared::game::EntityId;


pub struct Clients {
	pub clients: HashMap<SocketAddr, Client>,

	to_remove: Vec<SocketAddr>,
}

impl Clients {
	pub fn new() -> Clients {
		Clients {
			clients  : HashMap::new(),
			to_remove: Vec::new(),
		}
	}

	pub fn remove_inactive<F>(
		&mut self,
		    now_s    : f64,
		    timeout_s: f64,
		mut on_remove: F,
	)
		where F: FnMut(Client)
	{
		for (&address, client) in self.clients.iter() {
			if client.last_active_s + timeout_s < now_s {
				self.to_remove.push(address);
			}
		}

		for address in self.to_remove.drain(..) {
			if let Some(client) = self.clients.remove(&address) {
				info!(
					"Removing {} (last active: {}, time of removal: {})",
					address, client.last_active_s, now_s,
				);

				on_remove(client);
			}
		}
	}
}


#[derive(Debug)]
pub struct Client {
	pub ship_id      : EntityId,
	pub last_active_s: f64,
}
