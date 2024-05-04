use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct People {
    pub id: i32,
    pub lastname: String,
    pub middlename: String,
    pub firstname: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, PgConnection};

        pub async fn db() -> Result<PgConnection, ServerFnError> {
            Ok(PgConnection::connect("postgres://postgres:31N$t31n@localhost:5432/gulp").await.expect("Database Connection Error"))
        }
    }
}

#[server(GetPeople, "/api")]
pub async fn get_people() -> Result<Vec<People>, ServerFnError> {
    let req = use_context::<actix_web::HttpRequest>();

    if let Some(req) = req {
        println!("req.path = {:#?}", req.path());
    }
    use futures::TryStreamExt;

    let mut conn = db().await?;
    let mut people = Vec::new();
    let mut rows =
        sqlx::query_as::<_, People>("select id,firstname,lastname,middlename from people_table")
            .fetch(&mut conn);

    while let Some(row) = rows.try_next().await? {
        people.push(row);
    }

    Ok(people)
}

#[server(AddPeople, "/api")]
pub async fn add_people(
    lastname: String,
    firstname: String,
    middlename: String,
) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    std::thread::sleep(std::time::Duration::from_millis(1200));

    match sqlx::query(
        "insert into people_table (firstname, middlename, lastname) values ($1,$2,$3)",
    )
    .bind(firstname)
    .bind(middlename)
    .bind(lastname)
    .execute(&mut conn)
    .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(RemovePeople, "/api")]
pub async fn remove_people(id: i32) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    Ok(sqlx::query("delete from people_table where id = $1")
        .bind(id)
        .execute(&mut conn)
        .await
        .map(|_| ())?)
}
