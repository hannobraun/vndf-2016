struct Body {
	pos_x: f64,
	pos_y: f64,
}

enum Visual {
	RenderAsMissile,
	RenderAsShip,
}

ecs!(
	component(Body, bodies): Body,
	component(Visual, visuals): Visual,
	component(Score, scores): u32,

	entity(Missile: Body, Visual): |(pos_x, pos_y): (f64, f64)| {
		(
			Body { pos_x: pos_x, pos_y: pos_y },
			RenderAsMissile
		)
	},
	entity(Ship: Body, Visual, Player): |score: u32| {
		(
			Body { pos_x: 0.0, pos_y: 0.0 },
			RenderAsShip,
			score
		)
	},

	world(World: Missile, Ship),

	snapshot(Shared: World(Body, Visual))
)
