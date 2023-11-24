

// create a cronjob interface
pub struct CronJob {
    pub id : i32,
    // the cronjob name
    pub name: String,
    // the cronjob description
    pub schedule: String,
    //cron
    pub active: bool,
    pub(crate) created_at: String,
    pub (crate) updated_at: String,

}

// create a new cronjob interface
pub trait CronTrait {
    fn new() -> Self;

    fn run(&self);

    fn set_name(&mut self, name: String);

}
