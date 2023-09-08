mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use crate::task_screen::systems::*;

pub struct TaskScreenPlugin; 

impl Plugin for TaskScreenPlugin {
   fn build(&self, app: &mut App) {
       app.add_system(spawn_task_screen.in_schedule(OnEnter(AppState::TaskScreen)))
       .add_systems(
        (
            buttons_handler::<Display>,
            buttons_handler::<Visibility>,
        )
        .in_set(OnUpdate(AppState::TaskScreen))
     );
   } 
}