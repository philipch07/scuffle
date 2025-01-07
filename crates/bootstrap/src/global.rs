use std::sync::Arc;

use crate::config::{ConfigParser, EmptyConfig};

fn default_runtime_builder() -> tokio::runtime::Builder {
    let worker_threads = std::env::var("TOKIO_WORKER_THREADS")
        .unwrap_or_default()
        .parse::<usize>()
        .ok()
        .or_else(|| std::thread::available_parallelism().ok().map(|p| p.get()));

    let mut builder = if let Some(1) = worker_threads {
        tokio::runtime::Builder::new_current_thread()
    } else {
        tokio::runtime::Builder::new_multi_thread()
    };

    if let Some(worker_threads) = worker_threads {
        builder.worker_threads(worker_threads);
    }

    if let Ok(max_blocking_threads) = std::env::var("TOKIO_MAX_BLOCKING_THREADS")
        .unwrap_or_default()
        .parse::<usize>()
    {
        builder.max_blocking_threads(max_blocking_threads);
    }

    if !std::env::var("TOKIO_DISABLE_TIME")
        .unwrap_or_default()
        .parse::<bool>()
        .ok()
        .unwrap_or(false)
    {
        builder.enable_time();
    }

    if !std::env::var("TOKIO_DISABLE_IO")
        .unwrap_or_default()
        .parse::<bool>()
        .ok()
        .unwrap_or(false)
    {
        builder.enable_io();
    }

    if let Ok(thread_stack_size) = std::env::var("TOKIO_THREAD_STACK_SIZE").unwrap_or_default().parse::<usize>() {
        builder.thread_stack_size(thread_stack_size);
    }

    if let Ok(global_queue_interval) = std::env::var("TOKIO_GLOBAL_QUEUE_INTERVAL")
        .unwrap_or_default()
        .parse::<u32>()
    {
        builder.global_queue_interval(global_queue_interval);
    }

    if let Ok(event_interval) = std::env::var("TOKIO_EVENT_INTERVAL").unwrap_or_default().parse::<u32>() {
        builder.event_interval(event_interval);
    }

    if let Ok(max_io_events_per_tick) = std::env::var("TOKIO_MAX_IO_EVENTS_PER_TICK")
        .unwrap_or_default()
        .parse::<usize>()
    {
        builder.max_io_events_per_tick(max_io_events_per_tick);
    }

    builder
}

pub trait Global: Send + Sync + 'static {
    type Config: ConfigParser + Send + 'static;

    /// Builds the tokio runtime for the application.
    #[inline(always)]
    fn tokio_runtime() -> tokio::runtime::Runtime {
        default_runtime_builder().build().expect("runtime build")
    }

    /// Called before loading the config.
    #[inline(always)]
    fn pre_init() -> anyhow::Result<()> {
        Ok(())
    }

    /// Initialize the global.
    fn init(config: Self::Config) -> impl std::future::Future<Output = anyhow::Result<Arc<Self>>> + Send;

    /// Called when all services have been started.
    #[inline(always)]
    fn on_services_start(self: &Arc<Self>) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        std::future::ready(Ok(()))
    }

    /// Called when the shutdown process is complete, right before exiting the
    /// process.
    #[inline(always)]
    fn on_exit(
        self: &Arc<Self>,
        result: anyhow::Result<()>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        std::future::ready(result)
    }

    /// Called when a service exits.
    #[inline(always)]
    fn on_service_exit(
        self: &Arc<Self>,
        name: &'static str,
        result: anyhow::Result<()>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        let _ = name;
        std::future::ready(result)
    }
}

pub trait GlobalWithoutConfig: Send + Sync + 'static {
    #[inline(always)]
    fn tokio_runtime() -> tokio::runtime::Runtime {
        default_runtime_builder().build().expect("runtime build")
    }

    /// Initialize the global.
    fn init() -> impl std::future::Future<Output = anyhow::Result<Arc<Self>>> + Send;

    /// Called when all services have been started.
    #[inline(always)]
    fn on_services_start(self: &Arc<Self>) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        std::future::ready(Ok(()))
    }

    /// Called when the shutdown process is complete, right before exiting the
    /// process.
    #[inline(always)]
    fn on_exit(
        self: &Arc<Self>,
        result: anyhow::Result<()>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        std::future::ready(result)
    }

    /// Called when a service exits.
    #[inline(always)]
    fn on_service_exit(
        self: &Arc<Self>,
        name: &'static str,
        result: anyhow::Result<()>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        let _ = name;
        std::future::ready(result)
    }
}

impl<T: GlobalWithoutConfig> Global for T {
    type Config = EmptyConfig;

    #[inline(always)]
    fn tokio_runtime() -> tokio::runtime::Runtime {
        <T as GlobalWithoutConfig>::tokio_runtime()
    }

    #[inline(always)]
    fn init(_: Self::Config) -> impl std::future::Future<Output = anyhow::Result<Arc<Self>>> + Send {
        <T as GlobalWithoutConfig>::init()
    }

    #[inline(always)]
    fn on_service_exit(
        self: &Arc<Self>,
        name: &'static str,
        result: anyhow::Result<()>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        <T as GlobalWithoutConfig>::on_service_exit(self, name, result)
    }

    #[inline(always)]
    fn on_services_start(self: &Arc<Self>) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        <T as GlobalWithoutConfig>::on_services_start(self)
    }

    #[inline(always)]
    fn on_exit(
        self: &Arc<Self>,
        result: anyhow::Result<()>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        <T as GlobalWithoutConfig>::on_exit(self, result)
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use std::sync::Arc;
    use std::thread;

    use super::{Global, GlobalWithoutConfig};
    use crate::EmptyConfig;

    struct TestGlobal;

    impl Global for TestGlobal {
        type Config = ();

        async fn init(_config: Self::Config) -> anyhow::Result<std::sync::Arc<Self>> {
            Ok(Arc::new(Self))
        }
    }

    #[tokio::test]
    async fn default_global() {
        thread::spawn(|| {
            // To get the coverage
            TestGlobal::tokio_runtime();
        });

        assert!(matches!(TestGlobal::pre_init(), Ok(())));
        let global = TestGlobal::init(()).await.unwrap();
        assert!(matches!(global.on_services_start().await, Ok(())));

        assert!(matches!(global.on_exit(Ok(())).await, Ok(())));
        assert!(global.on_exit(Err(anyhow::anyhow!("error"))).await.is_err());

        assert!(matches!(global.on_service_exit("test", Ok(())).await, Ok(())));
        assert!(global.on_service_exit("test", Err(anyhow::anyhow!("error"))).await.is_err());
    }

    struct TestGlobalWithoutConfig;

    impl GlobalWithoutConfig for TestGlobalWithoutConfig {
        async fn init() -> anyhow::Result<std::sync::Arc<Self>> {
            Ok(Arc::new(Self))
        }
    }

    #[tokio::test]
    async fn default_global_no_config() {
        thread::spawn(|| {
            // To get the coverage
            <TestGlobalWithoutConfig as Global>::tokio_runtime();
        });

        assert!(matches!(TestGlobalWithoutConfig::pre_init(), Ok(())));
        <TestGlobalWithoutConfig as Global>::init(EmptyConfig).await.unwrap();
        let global = <TestGlobalWithoutConfig as GlobalWithoutConfig>::init().await.unwrap();
        assert!(matches!(Global::on_services_start(&global).await, Ok(())));

        assert!(matches!(Global::on_exit(&global, Ok(())).await, Ok(())));
        assert!(Global::on_exit(&global, Err(anyhow::anyhow!("error"))).await.is_err());

        assert!(matches!(Global::on_service_exit(&global, "test", Ok(())).await, Ok(())));
        assert!(Global::on_service_exit(&global, "test", Err(anyhow::anyhow!("error")))
            .await
            .is_err());
    }
}
