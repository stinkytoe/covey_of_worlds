use bevy::asset::LoadState;
use bevy::prelude::*;

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
                if let Some((entity, _)) = query.iter().find(|(_, handle)| handle.id() == *id) {
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
