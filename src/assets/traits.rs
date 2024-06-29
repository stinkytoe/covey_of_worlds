use bevy::prelude::*;
use std::fmt::Debug;
use thiserror::Error;

use crate::components::iid::Iid;

#[derive(Event)]
pub struct LdtkAssetLoadEvent<T: LdtkAsset> {
    pub(crate) entity: Entity,
    pub(crate) handle: Handle<T>,
}

pub trait LdtkAsset
where
    Self: Asset + Debug + Sized,
{
    fn on_create_system(
        mut commands: Commands,
        query: Query<(Entity, &Handle<Self>), Without<Iid>>,
        assets: Res<Assets<Self>>,
        asset_server: Res<AssetServer>,
        mut asset_event_writer: EventWriter<LdtkAssetLoadEvent<Self>>,
    ) {
        for (entity, handle) in query.iter() {
            if asset_server.is_loaded_with_dependencies(handle) {
                debug!("LdtkAsset loaded! entity: {entity:?}, handle: {handle:?}");

                let self_asset = assets.get(handle).expect("bad handle?");

                commands.entity(entity).insert(Iid(self_asset.iid()));

                asset_event_writer.send(LdtkAssetLoadEvent {
                    entity,
                    handle: handle.clone(),
                });
            }
        }
    }

    fn on_modified_system(
        // mut commands: Commands,
        mut asset_event_reader: EventReader<AssetEvent<Self>>,
        mut asset_event_writer: EventWriter<LdtkAssetLoadEvent<Self>>,
        query: Query<(Entity, &Handle<Self>)>,
    ) {
        for event in asset_event_reader.read() {
            match event {
                AssetEvent::Added { id } => {
                    debug!("AssetEvent::Added: {id:?}");
                }
                AssetEvent::Modified { id } => {
                    debug!("AssetEvent::Modified: {id:?}");

                    for (entity, handle) in query.iter().filter(|(_, handle)| handle.id() == *id) {
                        asset_event_writer.send(LdtkAssetLoadEvent {
                            entity,
                            handle: handle.clone(),
                        });
                    }
                }
                AssetEvent::Removed { id } => {
                    debug!("AssetEvent::Removed: {id:?}");
                }
                AssetEvent::Unused { id } => {
                    debug!("AssetEvent::Unused: {id:?}");
                }
                AssetEvent::LoadedWithDependencies { id } => {
                    debug!("AssetEvent::LoadedWithDependencies: {id:?}");
                }
            }
        }
    }

    fn iid(&self) -> String;
}

#[derive(Debug, Error)]
pub(crate) enum LdtkAssetChildLoaderError {
    #[error("Bad handle?")]
    BadHandle,
}

pub(crate) trait LdtkAssetChildLoader<Child>
where
    Child: LdtkAsset,
    Self: LdtkAsset,
{
    fn load_children_system(
        mut commands: Commands,
        mut events: EventReader<LdtkAssetLoadEvent<Self>>,
        children_query: Query<(Entity, &Iid), With<Handle<Child>>>,
        self_assets: Res<Assets<Self>>,
        child_assets: Res<Assets<Child>>,
    ) -> Result<(), LdtkAssetChildLoaderError> {
        for LdtkAssetLoadEvent { entity, handle } in events.read() {
            debug!("Loading Children for: {entity:?}");
            let children = self_assets
                .get(handle)
                .ok_or(LdtkAssetChildLoaderError::BadHandle)?
                .children();

            // child iid matches an entity: ignore

            // child matches no entity iid: spawn
            children
                .iter()
                .filter(|child| {
                    let child_asset = child_assets.get(*child).expect("bad handle?");
                    !children_query
                        .iter()
                        .any(|(_, iid)| child_asset.iid() == iid.0)
                })
                .for_each(|child| {
                    commands.entity(*entity).with_children(|parent| {
                        parent.spawn(child.clone());
                    });
                });

            // entity has no child with iid: despawn
            children_query
                .iter()
                .filter(|(_, iid)| {
                    !children.iter().any(|child| {
                        let child_asset = child_assets.get(child).expect("bad handle?");
                        child_asset.iid() == iid.0
                    })
                })
                .for_each(|(entity, _)| {
                    let mut ec = commands.entity(entity);
                    // ec.remove_parent();
                    ec.despawn();
                });
        }

        Ok(())
    }

    fn children(&self) -> Vec<Handle<Child>>;
}
