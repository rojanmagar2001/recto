#![warn(clippy::all, clippy::pedantic)]

use editor::Editor;

mod editor;

fn main() -> anyhow::Result<()> {
    let mut editor = Editor::default();

    editor.run()?;

    Ok(())
}
