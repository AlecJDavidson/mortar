// use crate::structs::Brick;
// use crate::structs::Db;
// use uuid::Uuid;
//
// impl Db {
//     pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
//         let pool = sqlx::postgres::PgPoolOptions::new()
//             .max_connections(5)
//             .connect(database_url)
//             .await?;
//
//         Ok(Self { pool })
//     }
//
//     pub async fn create_brick(&self, brick: &Brick) -> Result<Uuid, sqlx::Error> {
//         let id = Uuid::new_v4();
//
//         sqlx::query!(
//             r#"
//             INSERT INTO bricks (id, name, creation_time, language, source_path)
//             VALUES ($1, $2, NOW(), $3, $4)
//             "#,
//             id,
//             brick.name,
//             brick.language.to_string(),
//             brick.source_path
//         )
//         .execute(&self.pool)
//         .await?;
//
//         Ok(id)
//     }
// }
