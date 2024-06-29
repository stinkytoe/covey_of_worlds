use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::assets::project::ProjectAsset;
use crate::components::iid::Iid;

#[derive(SystemParam)]
pub struct LdtkProjectCommands<'w, 's> {
    // commands: Commands<'w, 's>,
    // asset_server: Res<'w, AssetServer>,
    project_assets: Res<'w, Assets<ProjectAsset>>,
    query: Query<'w, 's, (Entity, &'static Handle<ProjectAsset>, &'static Iid)>,
}

impl<'w, 's> LdtkProjectCommands<'w, 's> {
    pub fn iter(&self) -> impl Iterator<Item = &ProjectAsset> {
        self.project_assets.iter().map(|(_, project)| project)
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = (Entity, &ProjectAsset, &Iid)> {
        self.query.iter().map(|(entity, handle, iid)| {
            (
                entity,
                self.project_assets.get(handle).expect("bad handle?"),
                iid,
            )
        })
    }
}

pub trait LdtkProjectCommandsEx<'w>: Iterator<Item = &'w ProjectAsset> + Sized {
    fn with_iid(mut self, iid: &'w str) -> Option<&'w ProjectAsset> {
        self.find(|inner_project| inner_project.iid == iid)
    }
}

impl<'w, I: Iterator<Item = &'w ProjectAsset>> LdtkProjectCommandsEx<'w> for I {}

pub trait LdtkProjectCommandsEntityEx<'w>:
    Iterator<Item = (Entity, &'w ProjectAsset, &'w Iid)> + Sized
{
    fn with_iid(mut self, iid: &'w str) -> Option<(Entity, &'w ProjectAsset, &'w Iid)> {
        self.find(|(_, _, inner_iid)| {
            // inner_iid
            //     .filter(|inner_iid| inner_iid.0 == iid)
            //     .map(|_| (entity, asset, inner_iid))
            inner_iid.0 == iid
        })
    }
}

impl<'w, I: Iterator<Item = (Entity, &'w ProjectAsset, &'w Iid)>> LdtkProjectCommandsEntityEx<'w>
    for I
{
}
