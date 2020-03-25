use tokio_postgres::{row::Row, Client};

pub type DbError = String;

#[derive(Debug)]
pub struct ServerConfig {
    pub id: i64,
    pub server_id: i64,
    pub config: serde_json::Value,
}

impl ServerConfig {
    pub async fn get(client: &mut Client, server_id: i64) -> Result<Self, DbError> {
        Ok(Self::from_row(
            client
                .query_one(
                    "SELECT * FROM server_configs WHERE server_id = $1",
                    &[&server_id],
                )
                .await
                .map_err(|e| e.to_string())?,
        )?)
    }

    pub async fn create(
        client: &mut Client,
        server_id: i64,
        config: serde_json::Value,
    ) -> Result<Self, DbError> {
        Ok(Self::from_row(
            client
                .query_one(
                    "INSERT INTO (server_id, config) server_configs VALUES ($1, $2) RETURNING *",
                    &[&server_id, &config],
                )
                .await
                .map_err(|e| e.to_string())?,
        )?)
    }

    pub async fn update(
        client: &mut Client,
        server_id: i64,
        config: serde_json::Value,
    ) -> Result<Self, DbError> {
        Ok(Self::from_row(
            client
                .query_one(
                    "UPDATE server_configs SET config = $2 WHERE server_id = $1",
                    &[&server_id, &config],
                )
                .await
                .map_err(|e| e.to_string())?,
        )?)
    }

    // pub async fn delete(client: &mut Client, server_id: i64) -> Result<u64, DbError> {
    //     Ok(client
    //         .execute(
    //             "DELETE FROM server_config WHERE server_id = $1",
    //             &[&server_id],
    //         )
    //         .await
    //         .map_err(|e| e.to_string())?)
    // }

    fn from_row(row: Row) -> Result<Self, DbError> {
        Ok(Self {
            id: row.try_get("id").map_err(|e| e.to_string())?,
            server_id: row.try_get("server_id").map_err(|e| e.to_string())?,
            config: row.try_get("config").map_err(|e| e.to_string())?,
        })
    }
}
