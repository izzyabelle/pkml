use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    Frame,
};

use crate::{app::App, selvec::PointerVec};

#[derive(Debug, Default)]
pub struct Ui {
    pub pokelist: [ListState; 2],
    pub movelist: [ListState; 2],
    pub mode: IMode,
    pub log_idx: usize,
}

#[derive(Debug, Default)]
pub enum IMode {
    #[default]
    PokeList,
    MoveList,
    AI,
    AIMoveList,
}

impl App {
    /// Renders the user interface widgets.
    pub fn render(&mut self, frame: &mut Frame) {
        let border = BorderType::Plain;

        let [_, main_bar, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(20),
            Constraint::Fill(1),
        ])
        .areas(frame.size());

        let [_, pklist_1, mvlist_1, middle, mvlist_2, pklist_2, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Length(30),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Fill(1),
        ])
        .areas(main_bar);

        let [log, gameinfo] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(6)]).areas(middle);
        let pklist: [[Rect; 2]; 2] = [
            Layout::vertical([Constraint::Length(8), Constraint::Fill(1)]).areas(pklist_1),
            Layout::vertical([Constraint::Length(8), Constraint::Fill(1)]).areas(pklist_2),
        ];
        let mvlist: [[Rect; 2]; 2] = [
            Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).areas(mvlist_1),
            Layout::vertical([Constraint::Length(6), Constraint::Fill(1)]).areas(mvlist_2),
        ];

        self.render_pokelists(pklist, frame);
        self.render_movelists(mvlist, frame);
        self.render_log(log, frame);
        self.render_gameinfo(gameinfo, frame);
    }

    fn render_pokelists(&mut self, areas: [[Rect; 2]; 2], frame: &mut Frame) {
        for (i, area) in areas.iter().enumerate() {
            let block = Block::new()
                .title(Line::raw("Pokelist").centered())
                .borders(Borders::ALL)
                .border_type(BorderType::Plain);

            let items: Vec<ListItem> = self.games.players[i]
                .roster
                .data
                .iter()
                .enumerate()
                .map(|(k, pokemon)| match self.games.players[i].roster.active {
                    Some(active) => {
                        if active == k {
                            ListItem::from(format!("{}", pokemon.id)).fg(Color::Magenta)
                        } else {
                            ListItem::from(format!("{}", pokemon.id))
                        }
                    }
                    None => ListItem::from(format!("{}", pokemon.id)),
                })
                .collect();

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::new().fg(Color::Green))
                .highlight_symbol(">>")
                .highlight_spacing(HighlightSpacing::Always);

            frame.render_stateful_widget(list, area[0], &mut self.ui.pokelist[i]);

            let info = if let Some(k) = self.ui.pokelist[i].selected() {
                format!("{}", self.games.players[i].roster[k])
            } else if let Some(active) = self.games.players[i].roster.active() {
                format!("{}", active)
            } else {
                "No selection".to_string()
            };

            let block = Block::new()
                .title(Line::raw("Pokemon Info").centered())
                .borders(Borders::ALL)
                .border_type(BorderType::Plain);

            frame.render_widget(
                Paragraph::new(info)
                    .block(block)
                    .fg(Color::White)
                    .centered(),
                area[1],
            )
        }
    }

    fn render_gameinfo(&mut self, area: Rect, frame: &mut Frame) {
        let block = Block::new()
            .title(Line::raw("Game Info").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        frame.render_widget(
            Paragraph::new(format!("{}", self.games.state))
                .block(block)
                .fg(Color::White)
                .centered(),
            area,
        )
    }

    fn render_movelists(&mut self, areas: [[Rect; 2]; 2], frame: &mut Frame) {
        for (i, area) in areas.iter().enumerate() {
            let block = Block::new()
                .title(Line::raw("Movelist").centered())
                .borders(Borders::ALL)
                .border_type(BorderType::Plain);

            let items: Vec<ListItem> = if let Some(selection) = self.ui.pokelist[i].selected() {
                self.games.players[i].roster[selection]
                    .moves
                    .data
                    .iter()
                    .enumerate()
                    .map(
                        |(k, move_)| match self.games.players[i].roster[selection].moves.active {
                            Some(active) => {
                                if active == k {
                                    ListItem::from(format!("{}", move_.id)).fg(Color::Magenta)
                                } else {
                                    ListItem::from(format!("{}", move_.id))
                                }
                            }
                            None => ListItem::from(format!("{}", move_.id)),
                        },
                    )
                    .collect()
            } else {
                vec![]
            };

            let list = List::new(items)
                .block(block)
                .highlight_style(Style::new().fg(Color::Green))
                .highlight_symbol(">>")
                .highlight_spacing(HighlightSpacing::Always);

            frame.render_stateful_widget(list, area[0], &mut self.ui.movelist[i]);

            let info = if let Some(k) = self.ui.movelist[i].selected() {
                format!(
                    "{}",
                    self.games.players[i].roster[self.ui.pokelist[i].selected().unwrap()].moves[k]
                )
            } else {
                format!("No selection")
            };

            let block = Block::new()
                .title(Line::raw("Move Info").centered())
                .borders(Borders::ALL)
                .border_type(BorderType::Plain);

            frame.render_widget(
                Paragraph::new(info)
                    .block(block)
                    .fg(Color::White)
                    .centered(),
                area[1],
            )
        }
    }

    fn render_log(&mut self, area: Rect, frame: &mut Frame) {
        let mut disp_log = String::new();
        let log = self.games.log.last().unwrap();

        if log.len() > 22 + self.ui.log_idx {
            for i in 0..22 {
                disp_log.push_str(&format!("{}\n", log[i + self.ui.log_idx]));
            }
        } else {
            for ele in log[self.ui.log_idx..].iter() {
                disp_log.push_str(&format!("{}\n", ele));
            }
            for i in 0..22 - log[self.ui.log_idx..].len() {
                disp_log.push_str(&String::from("\n"))
            }
        }

        let block = Block::new()
            .title(Line::raw("Log").centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        frame.render_widget(
            Paragraph::new(disp_log)
                .block(block)
                .fg(Color::White)
                .wrap(Wrap { trim: false })
                .centered(),
            area,
        )
    }
}
