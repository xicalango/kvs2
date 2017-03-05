
use ::ui::UiResult;

use std::path::{
    Path,
    PathBuf,
};

use std::process::Command;

#[derive(Debug)]
pub struct Hooks {
    post_change: Option<PathBuf>
}

impl Hooks {
    pub fn load_from_dir<P: AsRef<Path>>(p: P) -> Hooks {
        let path = p.as_ref();
        let post_change_hook_path = path.join(".postChange");
        
        let post_change: Option<PathBuf> = Hooks::get_hook(post_change_hook_path);

        Hooks {
            post_change: post_change
        }
    }

    fn get_hook(path_buf: PathBuf) -> Option<PathBuf> {
        if !path_buf.exists() {
            return None;
        }

        if let Some(metadata) = path_buf.metadata().ok() {
            if !metadata.is_file() {
                return None;
            }

            return Some(path_buf);
        } else {
            return None;
        }
    }

    pub fn run_post_change_hook(&self, result: &UiResult) -> Result<bool, String> {
        match self.post_change {
            Some(ref post_change_hook) => {
                let status = try!(Command::new(post_change_hook).status().map_err(|e| e.to_string()));
                Ok(status.success())
            },

            None => Ok(false),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use ::ui::UiResult;

    #[test]
    fn test_load_from_dir() {
        let hooks = Hooks::load_from_dir("test/hooks");

        assert_eq!(Some(PathBuf::from("test/hooks/.postChange")), hooks.post_change);
    }

    #[test]
    fn test_run_post_change() {
        let hooks = Hooks::load_from_dir("test/hooks");

        let result = hooks.run_post_change_hook(&UiResult::Ok);

        assert_eq!(Ok(true), result);
    }
}

