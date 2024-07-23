use crate::wallet::{gen_sol_wallet, generate_and_save_mnemonic};
use solana_sdk::{signature::Keypair, signer::Signer};
// use spinoff::{Spinner, spinners, Color};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    text::{Spans, Span},
    Terminal,
};
use crossterm::{event::{self, Event, KeyCode}, execute, terminal::EnterAlternateScreen};
use anyhow::Error;

pub fn create_keypair() -> Result<Keypair, Error> {
    clearscreen::clear().expect("Failed to clear the screen.");
    let mut terminal = setup_terminal()?;

    let mut sp = spinoff::Spinner::new(
        spinoff::spinners::Dots,
        "Creating keypair, please wait...",
        spinoff::Color::Cyan,
    );

    sp.clear();

    let (mnemonic, _) = generate_and_save_mnemonic();
    let seed_phrase = mnemonic.to_string();

    // Simulate processing time
    // thread::sleep(Duration::from_secs(2));

    let (_, keypairs) = gen_sol_wallet(&mnemonic);
    if keypairs.is_empty() {
        return Err(anyhow::anyhow!("No keypairs generated."));
    }

    let mut text = vec![
        Spans::from(Span::styled("Here is a list of derived accounts with their addresses:", Style::default().fg(Color::Green))),
    ];

    for (i, pk) in keypairs.iter().enumerate() {
        text.push(Spans::from(Span::styled(format!("Account {}: {}", i + 1, pk.pubkey().to_string()), Style::default().fg(Color::White))));
        text.push(Spans::from(Span::styled(format!("Associated Private Key: {:?}", pk.to_base58_string()), Style::default().fg(Color::Cyan).add_modifier(Modifier::RAPID_BLINK))));
    }

    let mut selected_option = String::new();
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Max(6),
                        Constraint::Percentage(50),
                        Constraint::Max(3)
                    ]
                    .as_ref(),
                )
                .split(size);

                let seed_text = vec![
                    Spans::from(Span::styled("Keypair successfully created.", Style::default().fg(Color::Green))),
                    Spans::from(Span::styled("This is your generated seed phrase. Save it securely in a safe place.", Style::default().fg(Color::Yellow))),
                    Spans::from(Span::styled(seed_phrase.clone(), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))),
                ];
        
                let seed_paragraph = Paragraph::new(seed_text)
                    .block(Block::default().title("Seed Phrase").borders(Borders::ALL));
        
                f.render_widget(seed_paragraph, chunks[0]);
            let accounts_paragraph = Paragraph::new(text.clone())
                .block(Block::default().title("Derived Accounts").borders(Borders::ALL));

            f.render_widget(accounts_paragraph, chunks[1]);

            let input_instruction = vec![
                Spans::from(Span::styled("Which numbered one do you want to import?", Style::default().fg(Color::Yellow))),
                Spans::from(Span::raw(selected_option.clone())),
            ];

            let input_paragraph = Paragraph::new(input_instruction)
                .block(Block::default().title("Input"));

            f.render_widget(input_paragraph, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => break,
                KeyCode::Char(c) => selected_option.push(c),
                KeyCode::Backspace => { selected_option.pop(); },
                _ => {}
            }
        }
    }

    let number: usize = selected_option.trim().parse().unwrap_or_default();

    if number < keypairs.len() {
        Ok(keypairs[number].insecure_clone())
    } else {
        Err(anyhow::anyhow!("index must be less than 10"))
    }

}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Error> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}