use crate::cronjobs::cronInterface::{CronJob, CronTrait};



pub struct CronA(CronJob);
impl CronTrait for CronA {

    fn new() -> CronA {
        CronA(CronJob {
            id: 1,
            name: "TestCronA".to_string(),
            schedule: "* * * * *".to_string(),
            active: todo!(),
            created_at: todo!(),
            updated_at: todo!(),
        })
    }

    fn run(&self) {
        println!("TestCronA");
    }

    fn set_name(&mut self, name: String) {
        todo!()
    }
}