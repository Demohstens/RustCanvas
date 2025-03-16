
use std::rc::Rc;
use slint::{ModelRc, SharedString, VecModel};

slint::include_modules!();
fn main() {
    let app = App::new().unwrap();
    let weak_app = app.as_weak();
    app.on_open_file_dialog(move || {
        let picked_dir = rfd::FileDialog::new()
            .pick_folder();
        match picked_dir {
            Some(picked_dir) => {
                if let Some(app) = weak_app.upgrade() {
                    let contents = get_contents_of_dir(picked_dir.to_str().unwrap());
                    let shared_contents: Vec<SharedString> = contents.into_iter().map(SharedString::from).collect();
                    app.set_data(ModelRc::from(Rc::new(VecModel::from(shared_contents))));
                    app.set_dir(SharedString::from(picked_dir.to_str().unwrap()));
                }
            }
        None => {}
        }
        
    });
    app.run().unwrap();   
}


fn get_contents_of_dir(dir: &str) -> Vec<String> {
    let mut contents = vec![];
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                contents.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }
    contents
}