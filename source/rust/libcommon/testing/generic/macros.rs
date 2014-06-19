#![macro_escape]

#[macro_export]
macro_rules! wait_while(
	($condition:expr $action:block) => ({
		let start_time = ::time::precise_time_ns();
		while $condition {
			$action

			if ::time::precise_time_ns() - start_time > 1000000000 {
				fail!(
					"Condition \"{}\" still true after one second",
					stringify!($condition)
				);
			}
		}
	})
)
