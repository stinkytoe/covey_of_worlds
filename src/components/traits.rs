use bevy::prelude::*;
use thiserror::Error;

use crate::assets::traits::{LdtkAsset, LdtkAssetLoadEvent};

#[derive(Debug, Error)]
pub(crate) enum LdtkComponentError {
    // #[error("try_from_ldtk_asset failed!")]
    // TryFromLdtkAsset,
    #[error("Bad handle?")]
    BadHandle,
    #[error("Bad path?")]
    BadPath,
}

pub(crate) trait LdtkComponent<A>
where
    A: LdtkAsset,
    Self: Component + Sized,
{
    fn on_ldtk_asset_event_system(
        mut commands: Commands,
        mut events: EventReader<LdtkAssetLoadEvent<A>>,
        mut query: Query<&mut Self>,
        assets: Res<Assets<A>>,
    ) -> Result<(), LdtkComponentError> {
        for LdtkAssetLoadEvent { entity, handle } in events.read() {
            debug!("LdtkAssetLoadEvent: {entity:?}");

            let asset = assets.get(handle).ok_or(LdtkComponentError::BadHandle)?;

            Self::do_assign(&mut commands, *entity, &mut query, asset)?;
        }

        Ok(())
    }

    fn do_assign(
        commands: &mut Commands,
        entity: Entity,
        query: &mut Query<&mut Self>,
        asset: &A,
    ) -> Result<(), LdtkComponentError>;
}
