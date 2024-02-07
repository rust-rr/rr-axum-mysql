mod dev_db;

pub async fn dev_init() {
    dev_db::init_dev_db().await.unwrap();
}
