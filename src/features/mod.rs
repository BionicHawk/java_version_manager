use crate::models::constants::{self};

pub struct Context {
    pub java_home: String,
}

impl Context {

    pub fn load() -> Option<Context> {
        let mut settings = Context {
            java_home: String::from(""),
        };

        let java_home = Context::get_java_home();
        
        if java_home.is_none() {
            return None;
        } 
        settings.java_home = java_home.unwrap();

        return Some(settings);
    }
    
    pub fn set_java_home(&mut self, java_home: String) {
        self.java_home = java_home;
    }
    
    pub fn save_context(&self) {
        std::env::set_var(constants::JAVA_HOME, self.java_home.clone());
        println!("Saving context...");
    }
    
    fn get_java_home() -> Option<String> {
        let java_home = std::env::var(constants::JAVA_HOME);

        if let Err(error) = java_home {
            println!("[!] No JAVA_HOME environment variable found, please set it on your system\n\n{}", error);
            return None;
        }

        return Some(java_home.unwrap());
    }

}