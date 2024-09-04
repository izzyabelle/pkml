use crate::app::{App, AppResult};
use crate::effect::PlayerId;
use crate::game::MoveSelection;
use crate::ui::IMode;
use rand::seq::SliceRandom;
use rand::thread_rng;
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        KeyCode::Up => match app.ui.mode {
            IMode::PokeList => app.ui.pokelist[0].select_previous(),
            IMode::MoveList => app.ui.movelist[0].select_previous(),
            IMode::AI => app.ui.pokelist[1].select_previous(),
            IMode::AIMoveList => app.ui.movelist[1].select_previous(),
        },
        KeyCode::Down => match app.ui.mode {
            IMode::PokeList => app.ui.pokelist[0].select_next(),
            IMode::MoveList => app.ui.movelist[0].select_next(),
            IMode::AI => app.ui.pokelist[1].select_next(),
            IMode::AIMoveList => app.ui.movelist[1].select_next(),
        },
        KeyCode::Left => match app.ui.mode {
            IMode::MoveList => {
                app.ui.mode = IMode::PokeList;
                app.ui.movelist[0].select(None);
            }
            IMode::AI => {
                app.ui.mode = IMode::AIMoveList;
                app.ui.movelist[1].select_first();
            }
            _ => {}
        },
        KeyCode::Right => match app.ui.mode {
            IMode::PokeList => {
                app.ui.mode = IMode::MoveList;
                app.ui.movelist[0].select_first();
            }
            IMode::AIMoveList => {
                app.ui.mode = IMode::AI;
                app.ui.movelist[1].select(None);
            }
            _ => {}
        },
        KeyCode::Tab => match app.ui.mode {
            IMode::PokeList | IMode::MoveList => {
                app.ui.mode = IMode::AI;
                app.ui.pokelist[0].select(None);
                app.ui.movelist[0].select(None);
                app.ui.pokelist[1].select_first();
            }
            IMode::AI | IMode::AIMoveList => {
                app.ui.mode = IMode::PokeList;
                app.ui.pokelist[1].select(None);
                app.ui.movelist[1].select(None);
                app.ui.pokelist[0].select_first();
            }
        },
        KeyCode::Char('u') => app.ui.log_idx += 1,
        KeyCode::Char('d') => {
            if app.ui.log_idx > 0 {
                app.ui.log_idx -= 1
            }
        }
        KeyCode::Enter => match app.ui.mode {
            IMode::PokeList => {
                let _move = MoveSelection::Switch(app.ui.pokelist[0].selected().unwrap_or(7));
                if app
                    .games
                    .list_valid_inputs(&PlayerId::Player1)
                    .contains(&_move)
                {
                    app.games.player_mut(&PlayerId::Player1).inputs.push(_move);
                    app.games.execute_turn();
                }
            }
            IMode::MoveList => {
                let _move = MoveSelection::Move(app.ui.movelist[0].selected().unwrap_or(7));
                if app
                    .games
                    .list_valid_inputs(&PlayerId::Player1)
                    .contains(&_move)
                    && app.ui.pokelist[0].selected().unwrap_or(7)
                        == app
                            .games
                            .player(&PlayerId::Player1)
                            .roster
                            .active
                            .unwrap_or(8)
                {
                    app.games.player_mut(&PlayerId::Player1).inputs.push(_move);
                    app.games.execute_turn();
                }
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
