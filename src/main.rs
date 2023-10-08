use deadpool::managed;
use deadpool::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Error;
use surrealdb::sql::{Strand, Thing};
use serde::{Deserialize, Serialize};
use axum::{
    Json,
    Router,
    Extension,
    routing::get,
};
use std::net::SocketAddr;
use std::sync::Arc;

struct Manager {}

#[async_trait]
impl managed::Manager for Manager {
    type Type = Surreal<Client>;
    type Error = Error;
    
    async fn create(&self) -> Result<Surreal<Client>, Error> {
        let db = Surreal::new::<Ws>("127.0.0.1:8080").await.unwrap();
        db.signin(Root {
        username: "root",
        password: "root",
        })
        .await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        Ok(db)

    }
    
    async fn recycle(&self, _: &mut Surreal<Client>, _: &managed::Metrics) -> managed::RecycleResult<Error> {
        Ok(())
    }
}

type Pool = managed::Pool<Manager>;
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    id: Thing,
    name: Strand
}
async fn handle_request(pool: Extension<Arc<Pool>>) -> Json<Vec<Person>>{
    let conn = pool.get().await.expect("Failed to get connection from pool");
    let test: Vec<Person> = conn.select("person").await.unwrap();
    Json(test)
}
#[tokio::main]
async fn main() {
    let mgr = Manager {};
    let pool = Pool::builder(mgr).max_size(50).build().unwrap();
    let shared_pool = Arc::new(pool);


    let app = Router::new()
        .route("/endpoint", get(handle_request))
        .layer(Extension(shared_pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
       .serve(app.into_make_service())
       .await
       .unwrap();
}
