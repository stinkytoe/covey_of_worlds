use bevy::prelude::*;

use crate::assets::traits::LdtkAsset;
use crate::components::iid::Iid;
use crate::components::traits::LdtkComponent;
use crate::components::traits::LdtkComponentError;

#[derive(Asset, Reflect)]
pub struct ProjectAsset {
    pub bg_color: Color,
    pub external_levels: bool,
    pub iid: String,
    pub json_version: String,
    // value: ldtk::LdtkJson,
    #[reflect(ignore)]
    pub(crate) self_handle: Handle<ProjectAsset>,
}

impl LdtkAsset for ProjectAsset {}

impl LdtkComponent<ProjectAsset> for Iid {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        _: &mut Query<&mut Iid>,
        asset: &ProjectAsset,
    ) -> Result<(), LdtkComponentError> {
        let component = Iid(asset.iid.clone());
        commands.entity(entity).insert(component);
        Ok(())
    }
}

impl LdtkComponent<ProjectAsset> for Transform {
    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Transform>,
        asset: &ProjectAsset,
    ) -> Result<(), LdtkComponentError> {
        if let Ok(mut transform) = query.get_mut(entity) {
            transform.translation = Vec3::ZERO;
        } else {
            commands.entity(entity).insert(SpatialBundle::default());
        }

        Ok(())
    }
}
