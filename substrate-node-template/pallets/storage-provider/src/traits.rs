pub trait StorageInterface {
	type Value;
	fn get_param() -> Self::Value;
	fn set_param(v: Self::Value);
}
