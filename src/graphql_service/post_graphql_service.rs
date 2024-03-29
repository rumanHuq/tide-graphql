use serde_json::Value;
use http::status::StatusCode;
use serde::{Serialize, Deserialize};
use tide::{error::ResultExt, response, Context, EndpointResult};
use super::super::State;
use std::sync::Mutex;

use juniper::{
    self,
    object,
    RootNode,
    http::GraphQLRequest,
    FieldResult,
    GraphQLObject,
    GraphQLInputObject
};
use mongodb::{
    doc,
    db::{ThreadedDatabase},
    oid::ObjectId,
    bson,
    Bson
};
#[derive(GraphQLObject, Default, Serialize, Deserialize, Debug)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct Human {
    name: String,
    sex: String
}

#[derive(GraphQLInputObject)]
#[graphql(description="A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String
}


struct Query;
struct Mutation;
impl juniper::Context for State {}

#[object(Context = State)]
impl Query {
    fn movie(ctx: &State, id: String)->FieldResult<Human>{
        let collection = ctx.db.lock().unwrap().collection("users");
        let doc = doc! {
            "_id": ObjectId::with_string(&id[..]).unwrap()
        };
        let mut cursor = collection.find_one(Some(doc.clone()), None)
        .ok().expect("Failed to execute find.");
        if let Some(doc) = cursor {
            let json: Value = Bson::Document(doc).into();
            let u: Human = serde_json::from_value(json).unwrap();
            Ok(u)
        } else { Ok(Human::default()) }
    }
}

#[object(Context = State)]
impl Mutation {
    fn hola()->FieldResult<String> {
        Ok("moi".into())
    }
}

type Schema = RootNode<'static, Query, Mutation>;

pub async fn post_graphql_service(mut cx: Context<State>) -> EndpointResult {
    let query: GraphQLRequest = cx.body_json().await.client_err()?;
    let schema = Schema::new(Query, Mutation);
    let gql = query.execute(&schema, cx.state());
    let status = if gql.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    let mut resp = response::json(gql);
    *resp.status_mut() = status;
    Ok(resp)
}
