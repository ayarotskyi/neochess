use tokio::task::{JoinError, JoinSet};

pub struct JoinSetLimited<
    T: Send + 'static,
    U: std::future::Future<Output = T> + Send + 'static,
    V: Iterator<Item = U>,
> {
    tasks_iter: V,
    join_set: JoinSet<T>,
    limit: usize,
}

impl<T: Send + 'static, U: Future<Output = T> + Send + 'static, V: Iterator<Item = U>>
    JoinSetLimited<T, U, V>
{
    pub fn new(mut tasks_iter: V, limit: usize) -> Self {
        let mut join_set = JoinSet::new();

        for _ in 0..limit {
            if let Some(task) = tasks_iter.next() {
                join_set.spawn(task);
            } else {
                break;
            }
        }

        Self {
            join_set,
            tasks_iter,
            limit,
        }
    }

    pub async fn join_next(&mut self) -> Option<Result<T, JoinError>> {
        let next = self.join_set.join_next().await;

        while self.join_set.len() < self.limit
            && let Some(next_task) = self.tasks_iter.next()
        {
            self.join_set.spawn(next_task);
        }

        return next;
    }
}
