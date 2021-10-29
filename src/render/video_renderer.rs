use crate::errors::*;

use crate::render::ui_manager::UIManager;
use crate::ply_dir::PlyDir;

use crate::reader::read;
use std::sync::mpsc::channel;
use std::sync::Arc;
use crate::ply::Ply;

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
        // for event in self.window.conrod_ui().global_input().events() {
        //     match *event {
        //         Event::Raw(Input::Press(Button::Keyboard(Key::Space))) => {
        //             if is_stop {
        //                 is_stop = false
        //             } else {
        //                 is_stop = true
        //             }
        //         }
        //         _ => {}
        //     }
        // }

        // if !is_stop {
        //     frame = rx.recv().unwrap();
        // }

        frame = rx.recv().unwrap();
        match &frame {
            Ok(f) => {
                // uiManager.render_frame(f.get_points_as_ref(), &ids, &mut app);
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