pub trait Hashing {
	// transfer Block struc into bytes
	fn bytes(&self) -> Vec<u8>;

	fn hashing(&self) -> Vec<u8> {
		crypto_hash::digest(
			crypto_hash::Algorithm::SHA256, 
			&self.bytes())
	}
}