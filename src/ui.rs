use crate::emulator::Emulator;
use crate::disassembler;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

enum UiMode {
    Playing,
    Inspector,
}

pub fn run(mut emulator: Emulator, cycles_per_frame: u32) -> Result<(), io::Error> {
  
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut mode = UiMode::Playing;
    let mut last_timer_update = Instant::now();
    let timer_interval = Duration::from_millis(1000 / 60); 

    let result = run_app(&mut terminal, &mut emulator, cycles_per_frame, &mut mode, &mut last_timer_update, timer_interval);

  
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    emulator: &mut Emulator,
    cycles_per_frame: u32,
    mode: &mut UiMode,
    last_timer_update: &mut Instant,
    timer_interval: Duration,
) -> Result<(), io::Error> {
    let mut keys_pressed = Vec::new();
    
    loop {

        emulator.keypad = [false; 16];
        
        keys_pressed.clear();
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if handle_input(emulator, key, mode, &mut keys_pressed) {
                    return Ok(()); 
                }
            }
        }
        
        for key_idx in &keys_pressed {
            emulator.keypad[*key_idx] = true;
        }

        if last_timer_update.elapsed() >= timer_interval {
            emulator.update_timers();
            *last_timer_update = Instant::now();
        }

        if !emulator.paused {
            for _ in 0..cycles_per_frame {
                emulator.cycle();
            }
            emulator.update_ghost();
        }

        terminal.draw(|f| {
            match mode {
                UiMode::Playing => render_playing(f, emulator),
                UiMode::Inspector => render_inspector(f, emulator),
            }
        })?;

        std::thread::sleep(Duration::from_millis(16)); 
    }
}

fn handle_input(emulator: &mut Emulator, key: KeyEvent, mode: &mut UiMode, keys_pressed: &mut Vec<usize>) -> bool {
 
    match key.code {
        KeyCode::Esc => return true, 
        KeyCode::Char('i') | KeyCode::Char('I') => {
            *mode = match mode {
                UiMode::Playing => {
                    emulator.paused = true;
                    UiMode::Inspector
                }
                UiMode::Inspector => {
                    emulator.paused = false;
                    UiMode::Playing
                }
            };
            return false;
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            emulator.paused = !emulator.paused;
            return false;
        }
        _ => {}
    }

    if matches!(mode, UiMode::Inspector) {
        match key.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                emulator.cycle();
                return false;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                emulator.paused = false;
                return false;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                emulator.reset();
                return false;
            }
            KeyCode::Char('k') | KeyCode::Char('K') => {
                emulator.save_checkpoint();
                return false;
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                emulator.load_checkpoint();
                return false;
            }
            _ => {}
        }
    }

    // CHIP-8 keypad mapping (0-F)
    // Original:  1 2 3 C    Keyboard:  1 2 3 4
    //            4 5 6 D               Q W E R
    //            7 8 9 E               A S D F
    //            A 0 B F               Z X C V
    let key_map = [
        (KeyCode::Char('1'), 0x1),
        (KeyCode::Char('2'), 0x2),
        (KeyCode::Char('3'), 0x3),
        (KeyCode::Char('4'), 0xC),
        (KeyCode::Char('q'), 0x4),
        (KeyCode::Char('w'), 0x5),
        (KeyCode::Char('e'), 0x6),
        (KeyCode::Char('r'), 0xD),
        (KeyCode::Char('a'), 0x7),
        (KeyCode::Char('s'), 0x8),
        (KeyCode::Char('d'), 0x9),
        (KeyCode::Char('f'), 0xE),
        (KeyCode::Char('z'), 0xA),
        (KeyCode::Char('x'), 0x0),
        (KeyCode::Char('c'), 0xB),
        (KeyCode::Char('v'), 0xF),
    ];


    for (code, chip8_key) in key_map.iter() {
        if key.code == *code {
            if !keys_pressed.contains(chip8_key) {
                keys_pressed.push(*chip8_key);
            }
            return false;
        }
    }

    false
}

fn render_playing(f: &mut Frame, emulator: &Emulator) {
    let size = f.area();


    let display_area = centered_rect(70, 80, size);

    let block = Block::default()
        .title(" CHIP-8 Emulator ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(display_area);
    f.render_widget(block, display_area);


    let display_text = render_display_crt(&emulator.display, &emulator.ghost_display);
    let display_widget = Paragraph::new(display_text)
        .style(Style::default().fg(Color::Green));

    f.render_widget(display_widget, inner);


    let status_area = Rect {
        x: display_area.x,
        y: display_area.y + display_area.height,
        width: display_area.width,
        height: 1,
    };

    let status = if emulator.paused {
        " [PAUSED] Press I for Inspector | P to Resume | ESC to Quit "
    } else {
        " Press I for Inspector | P to Pause | ESC to Quit "
    };

    let status_widget = Paragraph::new(status)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    if status_area.y < size.height {
        f.render_widget(status_widget, status_area);
    }
}

fn render_inspector(f: &mut Frame, emulator: &Emulator) {
    let size = f.area();


    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(size);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(main_chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(main_chunks[1]);

    let cpu_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(top_chunks[1]);

    render_display_widget(f, emulator, top_chunks[0]);

    render_cpu_state(f, emulator, cpu_chunks[0]);

    render_stack(f, emulator, cpu_chunks[1]);

    render_disassembly(f, emulator, bottom_chunks[0]);

    render_controls(f, emulator, bottom_chunks[1]);
}

fn render_display_widget(f: &mut Frame, emulator: &Emulator, area: Rect) {
    let block = Block::default()
        .title(" Display ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let display_text = render_display_crt(&emulator.display, &emulator.ghost_display);
    let display_widget = Paragraph::new(display_text)
        .style(Style::default().fg(Color::Green));

    f.render_widget(display_widget, inner);
}

fn render_cpu_state(f: &mut Frame, emulator: &Emulator, area: Rect) {
    let block = Block::default()
        .title(" CPU State ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = vec![];

    lines.push(Line::from(vec![
        Span::styled("PC: ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("0x{:04X}", emulator.pc)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("I:  ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("0x{:04X}", emulator.i)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("SP: ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("0x{:02X}", emulator.sp)),
    ]));
    lines.push(Line::from(""));

    lines.push(Line::from(vec![
        Span::styled("DT: ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("{:3}", emulator.delay_timer)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("ST: ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("{:3}", emulator.sound_timer)),
    ]));
    lines.push(Line::from(""));

    for i in 0..8 {
        let left_reg = i;
        let right_reg = i + 8;
        lines.push(Line::from(vec![
            Span::styled(format!("V{:X}: ", left_reg), Style::default().fg(Color::Green)),
            Span::raw(format!("{:02X}  ", emulator.v[left_reg])),
            Span::styled(format!("V{:X}: ", right_reg), Style::default().fg(Color::Green)),
            Span::raw(format!("{:02X}", emulator.v[right_reg])),
        ]));
    }

    let cpu_widget = Paragraph::new(lines);
    f.render_widget(cpu_widget, inner);
}

fn render_stack(f: &mut Frame, emulator: &Emulator, area: Rect) {
    let block = Block::default()
        .title(" Stack ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = vec![];

    if emulator.sp == 0 {
        lines.push(Line::from(Span::styled("Empty", Style::default().fg(Color::DarkGray))));
    } else {
        for i in (0..emulator.sp.min(8)).rev() {
            let addr = emulator.stack[i as usize];
            let marker = if i == emulator.sp - 1 { ">" } else { " " };
            lines.push(Line::from(vec![
                Span::raw(marker),
                Span::styled(format!(" [{:X}] ", i), Style::default().fg(Color::Cyan)),
                Span::raw(format!("0x{:04X}", addr)),
            ]));
        }
    }

    let stack_widget = Paragraph::new(lines);
    f.render_widget(stack_widget, inner);
}

fn render_disassembly(f: &mut Frame, emulator: &Emulator, area: Rect) {
    let block = Block::default()
        .title(" Disassembly ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut lines = vec![];

    for i in 0..12 {
        let addr = emulator.pc + (i * 2);
        if addr >= 4094 {
            break;
        }

        let opcode = (emulator.memory[addr as usize] as u16) << 8
            | emulator.memory[addr as usize + 1] as u16;
        let disasm = disassembler::disassemble(opcode);

        let marker = if i == 0 { ">" } else { " " };
        let style = if i == 0 {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        lines.push(Line::from(vec![
            Span::raw(marker),
            Span::styled(format!(" {:04X}: ", addr), Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:04X} ", opcode), Style::default().fg(Color::Cyan)),
            Span::styled(disasm, style),
        ]));
    }

    let disasm_widget = Paragraph::new(lines);
    f.render_widget(disasm_widget, inner);
}

fn render_controls(f: &mut Frame, emulator: &Emulator, area: Rect) {
    let block = Block::default()
        .title(" Controls ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let checkpoint_status = if emulator.checkpoint.is_some() {
        "✓ Saved"
    } else {
        "✗ None"
    };

    let controls = vec![
        Line::from(vec![
            Span::styled("[S] ", Style::default().fg(Color::Yellow)),
            Span::raw("Step Forward"),
        ]),
        Line::from(vec![
            Span::styled("[C] ", Style::default().fg(Color::Yellow)),
            Span::raw("Continue"),
        ]),
        Line::from(vec![
            Span::styled("[R] ", Style::default().fg(Color::Yellow)),
            Span::raw("Reset"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("[K] ", Style::default().fg(Color::Yellow)),
            Span::raw("Save Checkpoint"),
        ]),
        Line::from(vec![
            Span::styled("[L] ", Style::default().fg(Color::Yellow)),
            Span::raw("Load Checkpoint"),
        ]),
        Line::from(vec![
            Span::styled("    ", Style::default()),
            Span::styled(checkpoint_status, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("[I] ", Style::default().fg(Color::Yellow)),
            Span::raw("Exit Inspector"),
        ]),
        Line::from(vec![
            Span::styled("[ESC] ", Style::default().fg(Color::Yellow)),
            Span::raw("Quit"),
        ]),
    ];

    let controls_widget = Paragraph::new(controls);
    f.render_widget(controls_widget, inner);
}

fn render_display_crt(display: &[bool; 2048], ghost: &[u8; 2048]) -> String {
    let mut output = String::with_capacity(DISPLAY_HEIGHT * (DISPLAY_WIDTH * 2 + 1));

    for y in 0..DISPLAY_HEIGHT {
        for x in 0..DISPLAY_WIDTH {
            let idx = y * DISPLAY_WIDTH + x;
            let pixel = display[idx];
            let ghost_val = ghost[idx];

            let char = if pixel {
                if y % 2 == 0 {
                    "██" 
                } else {
                    "▓▓" 
                }
            } else if ghost_val > 50 {
                "░░" 
            } else {
                "  " 
            };

            output.push_str(char);
        }
        output.push('\n');
    }

    output
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}


