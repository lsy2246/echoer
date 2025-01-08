use surrealdb::{engine::any::{connect,Any}, Surreal,opt::auth::Root};

use crate::utils::error::CustomResult;

pub struct Database {
    client: Surreal<Any>,
}

impl Database {
    pub async fn link() -> CustomResult<Self> {
        let client = connect("ws://172.30.113.67:5000").await?;

        // 使用root账户进行认证
        client
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;

        // 选择命名空间和数据库
        client.use_ns("echoer").await?;

        Ok(Self { client })
    }

    pub fn get_client(&self) -> &Surreal<Any> { 
        &self.client
    }
}
