use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::assets::project::ProjectAsset;
use crate::assets::traits::LdtkAsset;
use crate::components::iid::Iid;
use crate::components::tileset_rectangle::TilesetRectangle;
use crate::components::traits::LdtkComponent;
use crate::exports::field_instance::FieldInstance;

#[derive(Asset, Debug, Reflect)]
pub struct EntityAsset {
    pub grid: IVec2,
    pub identifier: String,
    pub anchor: Anchor,
    pub smart_color: Color,
    pub tags: Vec<String>,
    pub tile: Option<TilesetRectangle>,
    pub world_location: Option<Vec2>,
    pub def_uid: i64,
    pub field_instances: Vec<FieldInstance>,
    pub size: Vec2,
    pub iid: String,
    pub location: Vec2,
    #[reflect(ignore)]
    pub project: Handle<ProjectAsset>,
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
