
pub mod rand_func {
    use rand::Rng;

    pub fn rand_num() {
        let mut rng = rand::thread_rng();
        let n1: u8 = rng.gen();
        println!("random number: {}", n1);
    }

}
