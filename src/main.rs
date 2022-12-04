use rand_api::evaluate;
use reedline::{Reedline, DefaultPrompt, DefaultPromptSegment, Signal, PromptHistorySearchStatus, FileBackedHistory, ReedlineMenu, ColumnarMenu};

fn main() {
    let history = Box::new(
        FileBackedHistory::new(20)
    );
    let mut line_editor = Reedline::create()
        .with_history(history)
        .with_menu(ReedlineMenu::HistoryMenu(Box::new(ColumnarMenu::default())));
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("rand-api".to_owned()),
        right_prompt: DefaultPromptSegment::Empty
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