use bevy::ecs::schedule::States;




#[derive(States, Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum GameState{
    #[default]
    //LoadingGame,
    InGame,
    Paused
}