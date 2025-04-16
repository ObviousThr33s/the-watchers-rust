struct Element {
	id:i64,
	v: Vec<Element>
}

impl Element {
	fn add_to_array<T>(array: &mut Vec<T>, element: T) {
		array.push(element);
	}
}

