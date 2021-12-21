use crate::errors::*;

use crate::ply_dir::PlyDir;
use crate::render::ui_manager::UIManager;

use crate::ply::Ply;
use crate::reader::read;
use std::sync::mpsc::channel;
use std::sync::Arc;

pub fn render_video(ui_manager: &mut UIManager, ply_dir: PlyDir) -> Result<()> {
    let len = ply_dir.count();
    let paths = Arc::new(ply_dir.get_paths());

    let (tx, rx) = channel();
    let (paths_clone, tx) = (paths, tx);

    std::thread::spawn(move || {
        let mut index: usize = 0;
        loop {
            index += 1;
            let frame = read(paths_clone[index % len].to_str());
            tx.send(frame).unwrap();
        }
    });

    let mut frame = Ok(Ply::nothing());

    // self.window.conrod_ui_mut().theme = gui::theme();
    // let ids = gui::Ids::new(self.window.conrod_ui_mut().widget_id_generator());
    // let mut app = gui::InfoBar::new_closed_state();

    // let mut is_stop = false;

    while ui_manager.render() {
        frame = rx.recv().unwrap();
        match &frame {
            Ok(f) => {
                ui_manager.render_frame(f.get_points_as_ref());
            }
            Err(e) => {
                eprintln!("Problem with reading file:\n    {}", e);
                continue;
            }
        }
    }

    Ok(())
}
