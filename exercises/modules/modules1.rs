mod sausage_factory {
    // This function is private and can only be called within this module.
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // This function is public and can be called from outside this module.
    pub fn make_sausage() {
        let secret_recipe = get_secret_recipe();
        println!("Sausage made with secret recipe: {}", secret_recipe);
    }
}

fn main() {
    sausage_factory::make_sausage();
}