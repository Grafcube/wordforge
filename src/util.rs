use rand::Rng;

pub fn generate_id() -> u32 {
    rand::thread_rng().gen()
}
