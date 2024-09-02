use crate::app::{App, AppResult};
use crate::ui::IMode;
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
        _ => {}
    }
    Ok(())
}
