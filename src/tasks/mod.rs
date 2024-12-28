use tokio_cron_scheduler::Job;

// Tasks refer to periodic and/or scheduled operations with discord API and live monitoring of messages
// Tasks may be stateful and persist trought app lifetime
pub trait Task {
    fn job(&self) -> Job;
}

pub struct SimpleTask;

impl Task for SimpleTask {
    fn job(&self) -> Job {
        Job::new_async("* * * * * *", |_uuid, _js| {
            Box::pin(async move {
                println!("FOO");
            })
        })
        .unwrap()
    }
}

pub struct OtherTask;

impl Task for OtherTask {
    fn job(&self) -> Job {
        Job::new_async("* * * * * *", |_uuid, _js| {
            Box::pin(async move {
                println!("BAR");
            })
        })
        .unwrap()
    }
}
