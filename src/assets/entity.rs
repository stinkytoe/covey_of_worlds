use bevy::prelude::*;

use crate::assets::project::ProjectAsset;
use crate::assets::traits::LdtkAsset;
use crate::components::iid::Iid;
use crate::components::tileset_rectangle::TilesetRectangle;
use crate::components::traits::LdtkComponent;

#[derive(Asset, Reflect)]
pub struct EntityAsset {
    pub project_handle: Handle<ProjectAsset>,
    pub iid: String,
    pub tile: Option<TilesetRectangle>,
}

impl LdtkAsset for EntityAsset {}

impl LdtkComponent<EntityAsset> for Iid {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Iid>,
        asset: &EntityAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        let component = Iid(asset.iid.clone());
        commands.entity(entity).insert(component);
        Ok(())
    }
}

impl LdtkComponent<EntityAsset> for TilesetRectangle {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut TilesetRectangle>,
        asset: &EntityAsset,
    ) -> Result<(), crate::components::traits::LdtkComponentError> {
        match asset.tile.as_ref() {
            Some(tile) => {
                commands.entity(entity).insert(tile.clone());
            }
            None => {
                commands.entity(entity).remove::<TilesetRectangle>();
                commands.entity(entity).remove::<Sprite>();
                commands.entity(entity).remove::<Handle<Image>>();
            }
        };

        commands.entity(entity);

        Ok(())
    }
}
