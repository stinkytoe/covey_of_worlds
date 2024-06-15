use bevy::asset::LoadState;
use bevy::prelude::*;
use thiserror::Error;

#[derive(Component)]
pub struct LoadStub;

#[derive(Event)]
pub struct LdtkAssetLoadEvent<T: LdtkAsset> {
    pub(crate) entity: Entity,
    pub(crate) handle: Handle<T>,
}

pub trait LdtkAsset
where
    Self: Asset + Sized,
{
    fn on_create_system(
        mut commands: Commands,
        query: Query<(Entity, &Handle<Self>), Added<Handle<Self>>>,
    ) {
        for (entity, handle) in query.iter() {
            debug!("EntityAsset added! entity: {entity:?}, handle: {handle:?}");
            commands.entity(entity).insert(LoadStub);
        }
    }

    fn on_modified_system(
        mut commands: Commands,
        mut asset_event_reader: EventReader<AssetEvent<Self>>,
        query: Query<(Entity, &Handle<Self>)>,
    ) {
        for event in asset_event_reader.read() {
            if let AssetEvent::Modified { id } = event {
                for (entity, _) in query.iter().filter(|(_, handle)| handle.id() == *id) {
                    commands.entity(entity).insert(LoadStub);
                }
            };
        }
    }

    fn with_load_stub_system(
        mut commands: Commands,
        query: Query<(Entity, &Handle<Self>), With<LoadStub>>,
        asset_server: Res<AssetServer>,
        mut ldtk_asset_event_writer: EventWriter<LdtkAssetLoadEvent<Self>>,
    ) {
        for (entity, handle) in query.iter() {
            if let Some(LoadState::Loaded) = asset_server.get_load_state(handle) {
                debug!("EntityAsset finished loading! entity: {entity:?}, handle: {handle:?}");

                commands.entity(entity).remove::<LoadStub>();

                ldtk_asset_event_writer.send(LdtkAssetLoadEvent {
                    entity,
                    handle: handle.clone(),
                });
            }
        }
    }
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
        children_query: Query<(Entity, &Handle<Child>)>,
        self_assets: Res<Assets<Self>>,
    ) -> Result<(), LdtkAssetChildLoaderError> {
        for LdtkAssetLoadEvent { handle, .. } in events.read() {
            let mut children = self_assets
                .get(handle)
                .ok_or(LdtkAssetChildLoaderError::BadHandle)?
                .children();

            for (entity, handle) in children_query.iter() {
                if !children.contains(handle) {
                    commands.entity(entity).despawn_recursive();
                } else {
                    children.retain(|inner_handle| inner_handle != handle);
                }
            }

            for handle in children.iter() {
                commands.spawn(handle.clone());
            }
        }

        Ok(())
    }

    fn children(&self) -> Vec<Handle<Child>>;
}
