#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::door_confs::{ActiveModel, Entity, Model, Column};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub door_info: Option<serde_json::Value>,
}

impl Params {
    fn apply(&self, item: &mut ActiveModel) {
        item.door_info = Set(self.door_info.clone());
    }
}

async fn load_item(ctx: &AppContext, uid: String) -> Result<Model> {
    let item = Entity::find().filter(Column::Uid.eq(uid)).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let pid = auth.claims.pid.clone();
    format::json(Entity::find().filter(Column::UserPid.eq(pid)).all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(auth: auth::JWT, State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    println!("user pid: {}", &auth.claims.pid);
    let mut item = ActiveModel {
        user_pid: Set(auth.claims.pid.clone()),
        uid: Set(uuid::Uuid::new_v4()),
        ..Default::default()
    };
    params.apply(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn open(
    auth: auth::JWT,
    Path(uid): Path<String>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    format::empty_json()
}

#[debug_handler]
pub async fn get_one(Path(uid): Path<String>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, uid).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/doors/")
        .add("/", post(add))
        .add("/", get(list))
        .add("{uid}", get(get_one))
}
