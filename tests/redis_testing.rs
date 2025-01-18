use anyhow::Result;
use serial_test::serial;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage,
};
use testcontainers_example::Redis;
use tokio::sync::OnceCell;

type RedisContainer = ContainerAsync<GenericImage>;
static ONCE: OnceCell<RedisContainer> = OnceCell::const_new();

async fn start_redis() -> RedisContainer {
    GenericImage::new("redis", "7.2.4")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await
        .expect("Redis started")
}

async fn setup() -> Result<Redis> {
    let container = ONCE.get_or_init(start_redis).await;
    let host = container.get_host().await?;
    let port = container.get_host_port_ipv4(6379).await?;

    Redis::new(&format!("redis://{host}:{port}"))
}

#[tokio::test]
#[serial]
async fn list_keys_test() -> Result<()> {
    let redis = setup().await?;
    assert_eq!(redis.put("hallo", "bye").await?, ());
    assert_eq!(redis.get::<_, String>("hallo").await?, "bye");
    assert_eq!(redis.list_keys().await?, vec!["hallo"]);
    assert_eq!(redis.delete::<_, ()>("hallo").await?, ());
    Ok(())
}

#[tokio::test]
#[serial]
async fn crd_test() -> Result<()> {
    let redis = setup().await?;
    assert_eq!(redis.put("hallo", "bye").await?, ());
    assert_eq!(redis.get::<_, String>("hallo").await?, "bye");
    assert_eq!(redis.delete::<_, ()>("hallo").await?, ());
    assert_eq!(redis.list_keys().await?, Vec::<String>::new());
    Ok(())
}
