use rand_api::evaluate;
use reedline::{
    default_emacs_keybindings, DefaultPrompt, KeyCode, KeyModifiers, ListMenu, Reedline, ReedlineEvent, ReedlineMenu,
    Signal, DefaultHinter, Emacs, FileBackedHistory, DefaultPromptSegment
};
use nu_ansi_term::{Color, Style};

fn main() {
    let history = Box::new(FileBackedHistory::new(20));
    let mut keybindings = default_emacs_keybindings();
    keybindings.add_binding(
        KeyModifiers::CONTROL,
        KeyCode::Char('r'),
        ReedlineEvent::UntilFound(vec![
            ReedlineEvent::Menu("history".to_owned()),
            ReedlineEvent::MenuPageNext,
        ]),
    );
    keybindings.add_binding(
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        KeyCode::Char('r'),
        ReedlineEvent::MenuPagePrevious,
    );
    let edit_mode = Box::new(Emacs::new(keybindings));
    let mut line_editor =
        Reedline::create()
            .with_history(history)
            .with_menu(ReedlineMenu::HistoryMenu(Box::new(
                ListMenu::default().with_name("history"),
            )))
            .with_hinter(Box::new(
                DefaultHinter::default().with_style(Style::new().fg(Color::DarkGray)),
            ))
            .with_edit_mode(edit_mode);
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("rand-api".to_owned()),
        right_prompt: DefaultPromptSegment::Empty,
    };

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                println!("{}", evaluate(&buffer));
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}
