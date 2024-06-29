use bevy::prelude::*;
use std::fmt::Debug;
use thiserror::Error;

use crate::assets::traits::LdtkAsset;
use crate::assets::traits::LdtkAssetLoadEvent;

#[derive(Debug, Error)]
pub(crate) enum LdtkComponentError {
    // #[error("try_from_ldtk_asset failed!")]
    // TryFromLdtkAsset,
    #[error("Bad handle?")]
    BadHandle,
    // #[error("Bad path?")]
    // BadPath,
}

pub(crate) trait LdtkComponent<A>
where
    A: LdtkAsset,
    Self: Component + Debug + Sized,
{
    fn ldtk_asset_event(
        mut commands: Commands,
        mut events: EventReader<LdtkAssetLoadEvent<A>>,
        mut query: Query<&mut Self>,
        assets: Res<Assets<A>>,
    ) -> Result<(), LdtkComponentError> {
        for LdtkAssetLoadEvent { entity, handle } in events.read() {
            trace!("LdtkAssetLoadEvent: {entity:?}");

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
