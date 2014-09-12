use protocol::Perception;


#[test]
fn it_should_sort_entities_into_added_removed_and_created() {
	let perception = Perception::new(
		|&n| n,
		0u,
		vec!(0u, 1, 2, 3),
		vec!(0u, 1, 4, 5)
	);

	assert_eq!(vec!(4, 5), perception.added);
	assert_eq!(vec!(2, 3), perception.removed);
	assert_eq!(vec!(0, 1), perception.updated);
}

#[test]
fn it_should_handle_intermingled_cases() {
	let perception = Perception::new(
		|&n| n,
		0u,
		vec!(0u, 1, 2, 3),
		vec!(0u, 2, 3, 4)
	);

	assert_eq!(vec!(4)      , perception.added);
	assert_eq!(vec!(1)      , perception.removed);
	assert_eq!(vec!(0, 2, 3), perception.updated);

	let perception = Perception::new(
		|&n| n,
		0u,
		vec!(0u, 2, 3, 4),
		vec!(0u, 1, 2, 3)
	);

	assert_eq!(vec!(1)      , perception.added);
	assert_eq!(vec!(4)      , perception.removed);
	assert_eq!(vec!(0, 2, 3), perception.updated);
}

#[test]
fn it_should_handle_an_increase_in_entities() {
	let perception = Perception::new(
		|&n| n,
		0u,
		vec!(0u, 1),
		vec!(0u, 2, 3, 4)
	);

	assert_eq!(vec!(2, 3, 4), perception.added);
	assert_eq!(vec!(1)      , perception.removed);
	assert_eq!(vec!(0)      , perception.updated);
}

#[test]
fn it_should_handle_a_decrease_in_entities() {
	let perception = Perception::new(
		|&n| n,
		0u,
		vec!(0u, 1, 2, 3),
		vec!(0u, 4)
	);

	assert_eq!(vec!(4)      , perception.added);
	assert_eq!(vec!(1, 2, 3), perception.removed);
	assert_eq!(vec!(0)      , perception.updated);
}

#[test]
fn it_should_handle_unsorted_input() {
	let perception = Perception::new(
		|&n| n,
		0u,
		vec!(3u, 1, 0, 2),
		vec!(2u, 4, 0, 3)
	);

	assert_eq!(vec!(4)      , perception.added);
	assert_eq!(vec!(1)      , perception.removed);
	assert_eq!(vec!(0, 2, 3), perception.updated);
}
