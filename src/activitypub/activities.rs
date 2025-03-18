use std::sync::Arc;

use activitypub_federation::{
    activity_sending::SendActivityTask,
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::{
        AcceptType, CreateType, DeleteType, FollowType, LikeType, UndoType, UpdateType,
    },
    protocol::{context::WithContext, helpers::deserialize_one_or_many},
    traits::{ActivityHandler, Actor, Object},
};
use anyhow::Error;
use kuchiki::traits::TendrilSink;
use kuchikiki as kuchiki;
use serde::{Deserialize, Serialize};
use tracing::*;
use url::Url;

use crate::{
    graphql::WebContext,
    schema::{
        activitypub::{Note, NoteReply},
        Dive, User,
    },
    SITE_URL,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum PersonAcceptedActivities {
    Follow(Follow),
    UndoFollow(UndoFollow),
    Like(Like),
    UndoLike(UndoLike),
    CreateDiveComment(CreateDiveComment),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    actor: ObjectId<User>,
    object: ObjectId<User>,
    #[serde(rename = "type")]
    kind: FollowType,
    id: Url,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UndoFollow {
    actor: ObjectId<User>,
    #[serde(rename = "type")]
    kind: UndoType,
    object: Follow,
    id: Url,
}

impl Follow {
    pub fn new(actor: ObjectId<User>, object: ObjectId<User>, id: Url) -> Follow {
        Follow {
            actor,
            object,
            kind: Default::default(),
            id,
        }
    }
}

#[async_trait::async_trait]
impl ActivityHandler for Follow {
    type DataType = Arc<WebContext>;
    type Error = Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let local_user = self.object.dereference_local(data).await?;
        let follower = self.actor.dereference(data).await?;

        let id =
            format!("{}/accept/{}/{}", &*SITE_URL, local_user.id, follower.id).parse::<Url>()?;

        data.handle.new_follow(local_user.id, follower.id).await?;

        let accept = Accept::new(local_user.ap_id().into(), self, id);

        let activity = WithContext::new_default(accept);
        let sends = SendActivityTask::prepare(
            &activity,
            &local_user,
            vec![follower.shared_inbox_or_inbox()],
            data,
        )
        .await?;

        for send in sends {
            send.sign_and_send(data).await?;
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl ActivityHandler for UndoFollow {
    type DataType = Arc<WebContext>;
    type Error = Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let local_user = self.object.object.dereference_local(data).await?;
        let follower = self.actor.dereference(data).await?;

        data.handle
            .remove_follow(local_user.id, follower.id)
            .await?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Accept {
    actor: ObjectId<User>,
    object: Follow,
    #[serde(rename = "type")]
    kind: AcceptType,
    id: Url,
}

impl Accept {
    pub fn new(actor: ObjectId<User>, object: Follow, id: Url) -> Accept {
        Accept {
            actor,
            object,
            kind: Default::default(),
            id,
        }
    }
}

#[async_trait::async_trait]
impl ActivityHandler for Accept {
    type DataType = Arc<WebContext>;
    type Error = Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateDiveComment {
    actor: ObjectId<User>,
    object: NoteReply,
    #[serde(rename = "type")]
    kind: CreateOrUpdateType,
    id: Url,
}

#[async_trait::async_trait]
impl ActivityHandler for CreateDiveComment {
    type DataType = Arc<WebContext>;
    type Error = Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let note = self.object;

        let dive = note.in_reply_to.dereference_local(data).await?;
        let user = self.actor.dereference(data).await?;

        let plain_text = kuchiki::parse_html().one(note.content).text_contents();

        data.handle
            .new_external_comment(dive.id, user.id, &plain_text, note.id.inner().as_str())
            .await?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
    actor: ObjectId<User>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    to: Vec<Url>,
    object: Note,
    #[serde(rename = "type")]
    kind: CreateOrUpdateType,
    id: Url,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeletePost {
    actor: ObjectId<User>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    to: Vec<Url>,
    object: Url,
    #[serde(rename = "type")]
    kind: DeleteType,
    id: Url,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum CreateOrUpdateType {
    Create(CreateType),
    Update(UpdateType),
}

#[async_trait::async_trait]
impl ActivityHandler for CreatePost {
    type DataType = Arc<WebContext>;
    type Error = Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl CreatePost {
    pub async fn send(
        dive: Dive,
        inboxes: Vec<Url>,
        data: &Data<Arc<WebContext>>,
        update: bool,
    ) -> Result<(), Error> {
        let user = data.handle.user_details(dive.user_id).await?;

        let dive_id = dive.id;

        let mut note = dive.into_json(data).await?;

        let id = format!("{}/dives/{}/upsert", &*SITE_URL, dive_id).parse::<Url>()?;

        note.to.append(&mut inboxes.clone());

        let create = CreatePost {
            actor: note.attributed_to.clone(),
            to: inboxes.clone(),
            object: note,
            kind: if update {
                CreateOrUpdateType::Update(Default::default())
            } else {
                CreateOrUpdateType::Create(Default::default())
            },
            id,
        };

        debug!(
            "Sending {} to: {:?}",
            serde_json::to_string(&create)?,
            inboxes.iter().map(|val| val.as_str()).collect::<Vec<_>>()
        );

        let create_with_context = WithContext::new_default(create);
        let sends = SendActivityTask::prepare(&create_with_context, &user, inboxes, data).await?;
        for send in sends {
            if let Err(err) = send.sign_and_send(data).await {
                warn!("Error trying to send: {err:?}");
            }
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    actor: ObjectId<User>,
    object: ObjectId<Dive>,
    #[serde(rename = "type")]
    kind: LikeType,
    id: Url,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UndoLike {
    actor: ObjectId<User>,
    object: Like,
    #[serde(rename = "type")]
    kind: UndoType,
    id: Url,
}

#[async_trait::async_trait]
impl ActivityHandler for Like {
    type DataType = Arc<WebContext>;
    type Error = Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let user = self.actor.dereference(data).await?;
        let dive = self.object.dereference_local(data).await?;

        data.handle.like_dive(user.id, dive.id).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl ActivityHandler for UndoLike {
    type DataType = Arc<WebContext>;
    type Error = Error;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let user = self.actor.dereference(data).await?;
        let dive = self.object.object.dereference_local(data).await?;

        data.handle.unlike_dive(user.id, dive.id).await?;

        Ok(())
    }
}
