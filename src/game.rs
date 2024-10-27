use bevy::{log::tracing_subscriber::fmt::format, prelude::*};

/// The size of the game world.
pub const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

/// The possible states of the game.
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                OnEnter(GameState::GameOver),
                game_over.run_if(in_state(GameState::GameOver)),
            )
            .insert_resource(EventStartTime(Timer::from_seconds(10.0, TimerMode::Once)))
            .insert_resource(Score(0))
            .add_systems(
                FixedUpdate,
                (track_elapsed_time, display_score_and_time).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct EventStartTime(Timer);

#[derive(Resource)]
pub struct Score(pub u32);

fn track_elapsed_time(
    time: Res<Time>,
    mut timer: ResMut<EventStartTime>,
    mut state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        state.set(GameState::GameOver);
    }
}

#[derive(Component)]
struct UI;

fn remove_ui(commands: &mut Commands, ui_text_obj: Query<Entity, With<UI>>) {
    for entity in ui_text_obj.iter() {
        commands.entity(entity).despawn();
    }
}

fn display_score_and_time(
    score: Res<Score>,
    time: Res<EventStartTime>,
    mut commands: Commands,
    ui_text_obj: Query<Entity, With<UI>>,
) {
    remove_ui(&mut commands, ui_text_obj);

    let font_handle: Handle<Font> = Default::default();

    let score_text = Text::from_section(
        format!("Score: {}", score.0),
        TextStyle {
            font: font_handle.clone(),
            font_size: 50.0,
            color: Color::WHITE,
        },
    );

    commands
        .spawn(Text2dBundle {
            text: score_text,
            transform: Transform {
                translation: Vec3::Y * 300.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UI);

    let time_text = Text::from_section(
        format!("Time: {:.2}", time.0.remaining_secs()),
        TextStyle {
            font: font_handle,
            font_size: 50.0,
            color: Color::WHITE,
        },
    );

    commands
        .spawn(Text2dBundle {
            text: time_text,
            transform: Transform {
                translation: Vec3::Y * 250.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UI);
}

fn game_over(mut commands: Commands, score: Res<Score>, ui_text_obj: Query<Entity, With<UI>>) {
    remove_ui(&mut commands, ui_text_obj);
    println!("Game over!");

    let font_handle: Handle<Font> = Default::default();
    let game_over_text = Text::from_section(
        "Game Over",
        TextStyle {
            font: font_handle.clone(),
            font_size: 100.0,
            color: Color::WHITE,
        },
    );

    commands.spawn(Text2dBundle {
        text: game_over_text,
        transform: Transform {
            translation: Vec3::Y * 100.0,
            ..Default::default()
        },
        ..Default::default()
    });

    let score = Text::from_section(
        format!("Score {}", score.0),
        TextStyle {
            font: font_handle,
            font_size: 50.0,
            color: Color::WHITE,
        },
    );

    commands.spawn(Text2dBundle {
        text: score,
        transform: Transform {
            translation: Vec3::Y * -50.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
