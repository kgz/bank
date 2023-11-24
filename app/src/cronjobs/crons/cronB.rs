use crate::cronjobs::cronInterface::{CronJob, CronTrait};



pub struct CronB(CronJob);
impl CronTrait for CronB {

    fn new() -> CronB {
        CronB(CronJob {
            id: 2,
            name: "TestCron".to_string(),
            schedule: "* * * * *".to_string(),
            active: true,
            created_at: todo!(),
            updated_at: todo!(),

        })
    }

    fn run(&self) {
        println!("TestCronB");
    }

    fn set_name(&mut self, name: String) {
        todo!()
    }
}