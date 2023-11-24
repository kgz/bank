
use migrations::database::database::{self, Database, Ret};
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;
use super::{crons::{cronA::CronA, cronB::CronB}, cronInterface::CronTrait};


extern crate cron_tab;  

#[derive(Debug, EnumIter)]
pub enum Crons {
    CronA,
    CronB,
}



pub struct CronManager {
    pub cronjobs: Vec<Crons>,
}


impl CronManager {
    pub fn new() -> CronManager {
        CronManager {
            cronjobs: Vec::new(),
        }
    }

    //static
    pub fn add(&mut self, cronjob: Crons) {

        self.cronjobs.push(cronjob);
    }

    pub fn sync(&mut self, name: &str) {

        let db = database::new().unwrap();
        let r: Ret = db.query("SELECT * FROM cronjobs WHERE name = ?", Some(&[&name]));
        
        if r.results.len() < 1 {
            return
        }
        for row in r.results {
            let name:String = row.get("name").unwrap();
            // println!("name: {}", name);
            // let c = match name {
            //     "CronA" => Crons::CronA(CronA::new()),
            //     "CronB" => Crons::CronB(CronB::new()),
            //     _ => Crons::CronA(CronA::new()),
            // };

        }


        // self.add(Crons::TestCron(TestCron::new()));
    }


    pub fn run(&self) {

        // loop {
        //     println!("running cronjobs");
        //     std::thread::sleep(std::time::Duration::from_secs(10));
        // }

        //foreach Crons
        for job in Crons::iter() {
            match job {
                Crons::CronA => {
                    let c = CronA::new();
                    c.run();
                },
                Crons::CronB => {
                    let c = CronB::new();
                    c.run();
                },
            }
        }

    }

}