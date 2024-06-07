use tokio::{runtime::Runtime, task};

pub struct Daemon {
    rt: Runtime,
}

impl Daemon {
    pub fn new() -> Self {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()
            .unwrap();

        Self { rt }
    }

    pub fn run(&self, func: impl FnOnce() + Send + 'static) -> task::JoinHandle<()> {
        self.rt.spawn(async {
            func();
        })
    }
}
