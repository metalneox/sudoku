use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::app::{App, Difficulty};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    let min_width = 53u16;
    let min_height = 35u16;

    if size.width < min_width || size.height < min_height {
        draw_min_size_message(f, size);
        return;
    }

    let available_width = size.width.saturating_sub(min_width);
    let available_height = size.height.saturating_sub(min_height);
    let offset_x = available_width / 2;
    let offset_y = available_height / 2;

    let grid_area = Rect::new(offset_x, offset_y, min_width, min_height);

    let message_area = Rect::new(0, offset_y + min_height + 1, size.width, 3);

    let grid_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Plain)
        .title(" SUDOKU ")
        .title_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    f.render_widget(grid_block, grid_area);

    let inner_area = Rect::new(
        grid_area.x + 1,
        grid_area.y + 1,
        grid_area.width - 2,
        grid_area.height - 2,
    );

    draw_grid(f, inner_area, app);
    draw_message(f, message_area, app);
}

fn draw_grid(f: &mut Frame, area: Rect, app: &App) {
    let box_constraints = vec![Constraint::Length(11); 3];
    let box_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(box_constraints)
        .split(area);

    for box_row in 0..3 {
        let box_col_constraints = vec![Constraint::Length(17); 3];
        let box_col_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(box_col_constraints)
            .split(box_chunks[box_row]);

        for box_col in 0..3 {
            let box_area = box_col_chunks[box_col];

            let box_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .style(Style::default().fg(Color::White));

            f.render_widget(box_block, box_area);

            let inner_area = Rect::new(
                box_area.x + 1,
                box_area.y + 1,
                box_area.width - 2,
                box_area.height - 2,
            );

            let inner_constraints = vec![Constraint::Length(3); 3];
            let inner_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(inner_constraints)
                .split(inner_area);

            for cell_row in 0..3 {
                let cell_col_constraints = vec![Constraint::Length(5); 3];
                let cell_col_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(cell_col_constraints)
                    .split(inner_chunks[cell_row]);

                for cell_col in 0..3 {
                    let row = box_row * 3 + cell_row;
                    let col = box_col * 3 + cell_col;

                    let num = app.sudoku.grid[row][col];
                    let is_fixed = app.sudoku.is_fixed(row, col);
                    let is_cursor = (row, col) == app.cursor;
                    let is_empty = num.is_none();

                    let text = match num {
                        Some(n) => n.to_string(),
                        None => " ".to_string(),
                    };

                    let cell_style = if is_cursor {
                        Style::default()
                            .bg(Color::Yellow)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    } else if is_empty {
                        Style::default().bg(Color::DarkGray).fg(Color::DarkGray)
                    } else if is_fixed {
                        Style::default()
                            .bg(Color::Black)
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .bg(Color::Black)
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    };

                    let cell_block = Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Plain)
                        .style(Style::default().fg(Color::White));

                    let cell_paragraph = Paragraph::new(text)
                        .block(cell_block)
                        .style(cell_style)
                        .alignment(Alignment::Center);

                    f.render_widget(cell_paragraph, cell_col_chunks[cell_col]);
                }
            }
        }
    }
}

fn draw_message(f: &mut Frame, area: Rect, app: &App) {
    let key_style = Style::default()
        .bg(Color::Cyan)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD);

    let desc_style = Style::default().fg(Color::White);

    let (diff_text, diff_color) = match app.difficulty {
        Difficulty::Easy => (" EASY ", Color::Green),
        Difficulty::Medium => (" MEDIUM ", Color::Rgb(255, 165, 0)),
        Difficulty::Hard => (" HARD ", Color::Red),
        Difficulty::Master => (" MASTER ", Color::DarkGray),
    };

    let diff_style = Style::default()
        .bg(diff_color)
        .fg(if app.difficulty == Difficulty::Master {
            Color::White
        } else {
            Color::Black
        })
        .add_modifier(Modifier::BOLD);

    let spans = vec![
        Span::styled(" TAB ", key_style),
        Span::raw(" "),
        Span::styled(diff_text, diff_style),
        Span::raw("  "),
        Span::styled(" 1-9 ", key_style),
        Span::styled(" Input ", desc_style),
        Span::raw(" "),
        Span::styled(" ARROWS ", key_style),
        Span::styled(" Move ", desc_style),
        Span::raw(" "),
        Span::styled(" Z ", key_style),
        Span::styled(" Clear ", desc_style),
        Span::raw(" "),
        Span::styled(" N ", key_style),
        Span::styled(" New Play ", desc_style),
        Span::raw(" "),
        Span::styled(" ESC ", key_style),
        Span::styled(" Quit ", desc_style),
    ];

    let final_line = if let Some(msg) = &app.message {
        Line::from(vec![Span::styled(
            format!(" {} ", msg),
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )])
    } else {
        Line::from(spans)
    };

    let paragraph = Paragraph::new(final_line)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_min_size_message(f: &mut Frame, size: Rect) {
    let text = "Finestra troppo piccola.\nIngrandisci per giocare.";
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, size);
}
