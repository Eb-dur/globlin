use bevy::{prelude::*, utils::HashSet};



#[derive(Component)]
pub struct Tile;


#[derive(Component)]
pub struct Inventory;

#[derive(Resource, Default)]
pub struct SystemState{
    pub inventory: bool,
}

#[derive(Resource, Default)]
pub struct MyWorldCoords(pub Vec2);

#[derive(Resource, Default)]
pub struct Cursor(pub Handle<Image>);

#[derive(Resource, Default)]
pub struct Textures(pub HashSet<Handle<Image>>);

