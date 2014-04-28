use components:: Control;
use entities::Components;


pub trait Input<Context> {
	fn apply(&self, context: &Context, controls: &mut Components<Control>);
}
