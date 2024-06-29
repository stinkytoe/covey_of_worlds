use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetChildLoader;
use crate::assets::world::WorldAsset;
use crate::components::iid::Iid;
use crate::components::traits::LdtkComponent;
use crate::components::traits::LdtkComponentError;
use crate::defs::entity_definition::EntityDefinition;
use crate::defs::enum_definition::EnumDefinition;
use crate::defs::layer_definition::LayerDefinition;
use crate::defs::tileset_definition::TilesetDefinition;

#[derive(Asset, Debug, Reflect)]
pub struct ProjectAsset {
    pub bg_color: Color,
    pub external_levels: bool,
    pub iid: String,
    pub json_version: String,
    pub(crate) tileset_assets: HashMap<String, Handle<Image>>,
    pub(crate) background_assets: HashMap<String, Handle<Image>>,
    pub(crate) layer_defs: HashMap<i64, LayerDefinition>,
    pub(crate) entity_defs: HashMap<i64, EntityDefinition>,
    pub(crate) tileset_defs: HashMap<i64, TilesetDefinition>,
    pub(crate) enum_defs: HashMap<i64, EnumDefinition>,
    // #[reflect(ignore)]
    // pub(crate) self_handle: Handle<ProjectAsset>,
    #[reflect(ignore)]
    pub(crate) world_handles: Vec<Handle<WorldAsset>>,
}

impl LdtkAssetChildLoader<WorldAsset> for ProjectAsset {
    fn children(&self) -> Vec<Handle<WorldAsset>> {
        self.world_handles.clone()
    }
}

impl LdtkAsset for ProjectAsset {
    fn iid(&self) -> String {
        self.iid.clone()
    }
}

impl LdtkComponent<ProjectAsset> for Name {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Self>,
        asset: &ProjectAsset,
    ) -> Result<(), LdtkComponentError> {
        // let name = asset
        //     .self_handle
        //     .path()
        //     .ok_or(LdtkComponentError::BadPath)?
        //     .to_string();
        let component = Name::new(asset.iid.clone());
        commands.entity(entity).try_insert(component);
        Ok(())
    }
}

impl LdtkComponent<ProjectAsset> for Iid {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Iid>,
        asset: &ProjectAsset,
    ) -> Result<(), LdtkComponentError> {
        let component = Iid(asset.iid.clone());
        commands.entity(entity).try_insert(component);
        Ok(())
    }
}

impl LdtkComponent<ProjectAsset> for Transform {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Transform>,
        _: &ProjectAsset,
    ) -> Result<(), LdtkComponentError> {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = Vec3::ZERO;
        } else {
            commands.entity(entity).try_insert(SpatialBundle::default());
        }
        Ok(())
    }
}
