use crate::wallet::import_and_save_private_key;
use solana_sdk::signature::Keypair;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color as Cl, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn import_keypair() -> Keypair {
    clearscreen::clear().expect("Failed to clear the screen.");
    // Set up the terminal
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Failed to enter alternate screen");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create terminal");

    let mut private_key_as_str = String::new();

    // Draw the input interface
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                "Import Wallet Keypair",
                Style::default().fg(Cl::White).add_modifier(Modifier::BOLD),
            ));
        f.render_widget(block, size);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(size);

        let text = vec![
            Spans::from(Span::styled("Enter the private key of the wallet to be imported:", Style::default().fg(Cl::Magenta))),
            Spans::from(Span::styled(
                &private_key_as_str,
                Style::default().fg(Cl::Green),
            )),
        ];

        let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
        f.render_widget(paragraph, chunks[1]);
    }).expect("Failed to draw input interface");

    // Read user input
    loop {
        if let Event::Key(key) = event::read().expect("Failed to read event") {
            match key.code {
                KeyCode::Enter => break,
                KeyCode::Char(c) => private_key_as_str.push(c),
                KeyCode::Backspace => {
                    private_key_as_str.pop();
                }
                _ => {}
            }

            terminal.draw(|f| {
                let size = f.size();
                let block = Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        "Import Wallet Keypair",
                        Style::default().fg(Cl::White).add_modifier(Modifier::BOLD),
                    ));
                f.render_widget(block, size);

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(80),
                            Constraint::Percentage(10),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let text = vec![
                    Spans::from(Span::raw("Enter the private key of the wallet to be imported:")),
                    Spans::from(Span::styled(
                        &private_key_as_str,
                        Style::default().fg(Cl::Green),
                    )),
                ];

                let paragraph = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
                f.render_widget(paragraph, chunks[1]);
            }).expect("Failed to draw input interface");
        }
    }

    disable_raw_mode().expect("Failed to disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )
    .expect("Failed to leave alternate screen");
    terminal.show_cursor().expect("Failed to show cursor");

    let mut sp = spinoff::Spinner::new(
        spinoff::spinners::Dots,
        "Importing keypair, please wait...",
        spinoff::Color::White,
    );

    let wallet_keypair = Keypair::from_base58_string(&private_key_as_str);
    sp.update(spinoff::spinners::Dots, "Saving Keypair in File", spinoff::Color::Blue);
    import_and_save_private_key(&private_key_as_str);
    sp.clear();

    wallet_keypair
}