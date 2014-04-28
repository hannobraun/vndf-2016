use components:: Control;
use entities::Components;


pub trait Input {
	fn apply(&self, controls: &mut Components<Control>);
}
