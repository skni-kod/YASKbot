// Tasks refer to periodic and/or scheduled operations with discord API and live monitoring of messages
// Tasks may be stateful and persist trought app lifetime
pub trait Task {
    async fn execute(&mut self);
    async fn get_crontab(&self);
}