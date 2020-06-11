use actix_cors::Cors;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::{Object};
use async_graphql::{EmptyMutation, EmptySubscription, Result, Schema, Context, FieldResult, Json};
use bson::oid::ObjectId;
use async_graphql_actix_web::{GQLRequest, GQLResponseStream};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Criteria {
    pub name: String,
}

#[async_graphql::InputObject]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct QueryInput {
    #[field(default)]
    pub criteria: Json<Criteria>,
    #[field(default = 20)]
    pub limit: i32,
    #[field(default)]
    pub skip: i32,
}

struct Record {
    id: Option<ObjectId>
}

#[async_graphql::Object(desc = "Record")]
impl Record {
    async fn id(&self) -> Option<ObjectId> {
        self.id.clone()
    }
}

struct Query;

#[Object]
impl Query {
    async fn records(
        &self,
        ctx: &Context<'_>,
        #[arg(default)] query: QueryInput,
    ) -> FieldResult<Vec<Record>> {
        Ok(Vec::new())
    }
}

pub async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

async fn index(schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>, req: GQLRequest) -> GQLResponseStream {
    req.into_inner().execute_stream(&schema).await.into()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    println!("Playground: http://localhost:8000/playground");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default())
            .data(schema.clone())
            .route("/playground", web::get().to(index_playground))
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
