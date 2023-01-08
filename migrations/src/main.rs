use ::migrations::run_migrations;

pub mod migrations {
    // pub mod a;
    pub mod files{
        // automod::dir!("src/migrations/files"); 
    }
}



fn main(){
    run_migrations();


}