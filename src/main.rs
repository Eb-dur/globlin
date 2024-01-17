mod components;
use bevy::{prelude::*, window::PrimaryWindow};
use components::*;
use bevy::sprite::Sprite;
use glob::glob;

const CAMERA_SCALE_FACTOR: f32 = 1.06;
const CAMERA_PAN_SPEED: f32 = 500.;
const BLOCK_SIZE: f32 = 64.;
const BLOCK_GAP: f32 = 4.;
const WINDOW_SIZE: (f32, f32) = (1280 as f32, 960 as f32);



/*
/ Tanke: Har en loop som hittar vilken ruta jag har tryckt på
/        Den ändrar sedan id för den "rutan" och vilken bild det ska va
/       
/        Det måste finnas något som håller koll på vilka bilder som finns
/        Kanske göra en rutin för att generera alla rutsorter
/
/        I framtiden: automatisk hörnutjämning
/
/        Kanske lättast att göra med bara bilder och visa upp dem genom att 
/        direkt.
/
*/





fn setup(mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut state: ResMut<SystemState>,
){
    commands.spawn(Camera2dBundle::default(),);
    state.inventory = false;


    let xvec = Vec3::new(BLOCK_SIZE + BLOCK_GAP , 0. ,0.);
    let yvec = Vec3::new(0. , BLOCK_SIZE + BLOCK_GAP ,0.);

    
    for y in 0..64{
        for x  in 0 .. 64{
            let texture:Handle<Image> = asset_server.load("white.png");
            commands.spawn((SpriteBundle {
                sprite: Sprite { custom_size: Some(Vec2::new(64.,64.)), ..default()},
                texture,
                transform: Transform{translation:x as f32*xvec + y as f32*yvec, ..default()}, 
                ..default()
            },
            Tile),

        );

        }

    }

}

fn inventory_setup(
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Textures>,
    mut commands: Commands

)
{   

    commands.spawn((SpriteBundle {
        sprite: Sprite{color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(WINDOW_SIZE.0, WINDOW_SIZE.1)),
            ..default()
    },
    transform: Transform::from_translation(Vec3::new(0 as f32, 0 as f32, 1 as f32)),
    visibility: Visibility::Hidden,
    ..default()
    }, Inventory));

    let mut placement_x: f32 = 0. + 0.05*WINDOW_SIZE.0 - WINDOW_SIZE.0/2.;
    let mut placement_y: f32 = 0. - 0.05*WINDOW_SIZE.1 + WINDOW_SIZE.1/2.;
    
    for entry in glob("**/assets/*.png").unwrap() {
        if let Ok(path) = entry{
            let p = path.file_name().unwrap();
            let img:Handle<Image> = asset_server.load(p.to_os_string().into_string().unwrap());
            textures.0.insert(img.clone());
            commands.spawn((SpriteBundle {
                sprite: Sprite{
                    custom_size: Some(Vec2::new(BLOCK_SIZE, BLOCK_SIZE)),
                    ..default()
            },
                texture:img,
                transform: Transform::from_translation(Vec3::new(placement_x, placement_y, 1.)),
                visibility: Visibility::Hidden,
                ..default()
            }, Inventory));
        
        }
        placement_x += BLOCK_GAP + BLOCK_SIZE;
        if placement_x > 0. - 0.05*WINDOW_SIZE.0 + WINDOW_SIZE.0/2.{
            placement_y += BLOCK_GAP + BLOCK_SIZE;
            placement_x = 0. + 0.05*WINDOW_SIZE.0 - WINDOW_SIZE.0/2.;
        }
    }



}

fn camera_movement(
    mut camera: Query<(&mut Transform,With<Camera>, Without<Inventory>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut sys: ResMut<SystemState>,
    mut inv: Query<(&mut Transform,&mut Visibility, With<Inventory>)>
    )
    {
    for (mut transform, _, _) in &mut camera{
        if input.pressed(KeyCode::W){
            if !sys.inventory{
                transform.translation.y += CAMERA_PAN_SPEED*transform.scale.y * time.delta_seconds();
                for mut part in inv.iter_mut(){
                    part.0.translation.y += CAMERA_PAN_SPEED*transform.scale.y * time.delta_seconds();
                }

            }

        }
        if input.pressed(KeyCode::S){
            if !sys.inventory{
                transform.translation.y -= CAMERA_PAN_SPEED*transform.scale.y * time.delta_seconds();
                for mut part in inv.iter_mut(){
                    part.0.translation.y -= CAMERA_PAN_SPEED*transform.scale.y * time.delta_seconds();
                }
            }
        }
        if input.pressed(KeyCode::D){
            if  !sys.inventory{
                transform.translation.x += CAMERA_PAN_SPEED*transform.scale.x * time.delta_seconds();
                for mut part in inv.iter_mut(){
                    part.0.translation.x += CAMERA_PAN_SPEED*transform.scale.x * time.delta_seconds();
                }
            }
        }
        if input.pressed(KeyCode::A){
            if  !sys.inventory{
                transform.translation.x -= CAMERA_PAN_SPEED*transform.scale.x * time.delta_seconds();
                for mut part in inv.iter_mut(){
                    part.0.translation.x -= CAMERA_PAN_SPEED*transform.scale.x * time.delta_seconds();
                }
            }
        }
        // IN
        if input.pressed(KeyCode::Q){
            if !sys.inventory{
                let old_loc = transform.translation.clone();
                transform.scale /= CAMERA_SCALE_FACTOR;
                for mut part in inv.iter_mut(){
                    let offset = part.0.translation - old_loc;
                    part.0.scale /= CAMERA_SCALE_FACTOR;
                    part.0.translation = transform.translation + offset/CAMERA_SCALE_FACTOR;
                }
            }
        }
        // UT
        if input.pressed(KeyCode::E){
            if !sys.inventory{
                let old_loc = transform.translation.clone();
                transform.scale *= CAMERA_SCALE_FACTOR;
                for mut part in inv.iter_mut(){
                    let offset = part.0.translation - old_loc;
                    part.0.scale *= CAMERA_SCALE_FACTOR;
                    part.0.translation = transform.translation + offset*CAMERA_SCALE_FACTOR;
                }
            }
        }
        if input.just_released(KeyCode::R){
            sys.inventory = !sys.inventory;
            if sys.inventory{
                for mut part in inv.iter_mut(){
                    *part.1 = Visibility::Visible;
                }
            }
            else {
                for mut part in inv.iter_mut(){
                    *part.1 = Visibility::Hidden;
                }
            } 
        }
    }
}

fn at_block(mx:f32, my:f32 , x:f32, y:f32) -> bool{
    println!("Mx{} My{} X{} Y{}", mx,my,x,y);
    if x - BLOCK_SIZE/2 as f32 <= mx && mx <= x + BLOCK_SIZE/2 as f32{
        if y - BLOCK_SIZE/2 as f32 <= my && my <= y + BLOCK_SIZE/2 as f32{
            return true;
        }
        return false;

    }
    return false;
}

/*
 Borrowed from https://bevy-cheatbook.github.io/cookbook/cursor2world.html
*/
fn my_cursor_system(
    mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
        //eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}


fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    world_pos: Res<MyWorldCoords>,
    mut sys: ResMut<SystemState>,
    mut blocks: Query<(&mut Handle<Image>, & Transform, With<Tile>, Without<Inventory>)>,
    mut inventory: Query<(&Handle<Image>, &Transform,&mut Visibility,  With<Inventory>)>,
    mut cursor: ResMut<Cursor>
    
)
{
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released 
            if !sys.inventory{
                for (mut img, coords, _, _) in blocks.iter_mut(){
                    if at_block(world_pos.0.x, world_pos.0.y, coords.translation.x, coords.translation.y){
                        *img = cursor.0.clone();
                    }
                }
            }
            else {
                for (img, coords, _, _) in inventory.iter_mut(){
                    if at_block(world_pos.0.x, world_pos.0.y, coords.translation.x, coords.translation.y){
                        cursor.0 = img.clone();
                        sys.inventory = !sys.inventory;
                        

                    }
                }
                if sys.inventory{
                    for (_,_,mut vis,_) in inventory.iter_mut(){
                        *vis = Visibility::Visible;
                    }
                }
                else {
                    for (_,_,mut vis,_) in inventory.iter_mut(){
                        *vis = Visibility::Hidden;
                    }
                } 
            }

    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down

    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}



fn main() {
    App::new()
    .add_plugins(DefaultPlugins
                            .set(ImagePlugin::default_nearest())
                            .set(WindowPlugin { 
                                primary_window: Some(Window {
                                    title: "Projec globlin".into(),
                                    resolution: WINDOW_SIZE.into(),
                                    resizable: false,
                                    ..default()
                                }),
                                ..default()
                            })
                            .build(),
    )
    .init_resource::<MyWorldCoords>()
    .init_resource::<SystemState>()
    .init_resource::<Textures>()
    .init_resource::<Cursor>()
    .add_systems(Startup, (setup,
                                            inventory_setup,))
    .add_systems(Update, (camera_movement,
                                            my_cursor_system,
                                            ))
    .add_systems(Update,mouse_button_input)
    .run();


}