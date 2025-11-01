#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::door_confs::{ActiveModel, Entity, Model, Column};
use crate::models::_entities::user_doors;

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
    let user_pid = auth.claims.pid.to_string();
    let door_links = user_doors::Entity::find()
        .filter(user_doors::Column::UserPid.eq(user_pid))
        .all(&ctx.db)
        .await?;
    if door_links.is_empty() {
        return format::json(Vec::<Model>::new());
    }
    let door_uids: Vec<String> = door_links.into_iter().map(|link| link.door_uid).collect();
    let doors = Entity::find()
        .filter(Column::Uid.is_in(door_uids))
        .all(&ctx.db)
        .await?;
    format::json(doors)
}

#[debug_handler]
pub async fn add(auth: auth::JWT, State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Response> {
    let mut item = ActiveModel {
        uid: Set(uuid::Uuid::new_v4().to_string()),
        ..Default::default()
    };
    params.apply(&mut item);
    let item = item.insert(&ctx.db).await?;
    let user_pid = auth.claims.pid.clone();
    let link = user_doors::ActiveModel {
        user_pid: Set(user_pid),
        door_uid: Set(item.uid.to_string()),
        ..Default::default()
    };
    link.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn open(
    auth: auth::JWT,
    Path(uid): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_pid = auth.claims.pid.to_string();
    let door_link = user_doors::Entity::find()
        .filter(user_doors::Column::UserPid.eq(user_pid))
        .filter(user_doors::Column::DoorUid.eq(uid.clone()))
        .one(&ctx.db)
        .await?;
    if door_link.is_none() {
        return Err(Error::NotFound);
    }

    let door = load_item(&ctx, uid).await?;
    format::json(door)
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
        .add("/open/{door_uid}", post(open))
        .add("{door_uid}", get(get_one))
}
