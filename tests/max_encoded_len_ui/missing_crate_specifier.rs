use axia_scale_codec::{Encode, MaxEncodedLen};

#[derive(Encode, MaxEncodedLen)]
#[codec(axia_scale_codec)]
struct Example;

fn main() {
	let _ = Example::max_encoded_len();
}
