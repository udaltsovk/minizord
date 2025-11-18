use std::time::Duration;

use mobc::{Connection, Manager, Pool, async_trait};

#[async_trait]
pub trait MobcPool<M: Manager> {
    #[tracing::instrument(
        name = "MobcPool::new_pool",
        skip_all,
        level = "debug"
    )]
    fn new_pool(max_open: u64, manager: M) -> Pool<M> {
        Pool::builder()
            .max_open(max_open)
            .max_idle(7)
            .test_on_check_out(true)
            .max_lifetime(Some(Duration::from_secs(60 * 60)))
            .build(manager)
    }

    async fn get(&self) -> Result<Connection<M>, mobc::Error<M::Error>>;
}
