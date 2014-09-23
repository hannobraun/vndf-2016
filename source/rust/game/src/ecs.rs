use physics::Body;


#[deriving(Clone, Decodable, Encodable, PartialEq, Show)]
pub enum Visual {
	ShowAsMissile,
	ShowAsShip
}


world!(
	Missile(Body, Visual): |body: Body| {
		(body, ShowAsMissile)
	}
	Ship(Body, Visual): |body: Body| {
		(body, ShowAsShip)
	}
)
