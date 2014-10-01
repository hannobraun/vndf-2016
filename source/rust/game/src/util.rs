use cgmath::{
	ApproxEq,
	EuclideanVector,
	Line3,
	Point,
	Point3,
};


pub fn is_on_line(line: Line3<f64>, point: Point3<f64>) -> bool {
	if line.origin.approx_eq(&point) {
		return true;
	}

	let origin_to_dest  = line.dest.to_vec() - line.origin.to_vec();
	let origin_to_point = point.to_vec() - line.origin.to_vec();

	return origin_to_dest.normalize().approx_eq(&origin_to_point.normalize());
}
