
use ::ui::UiResult;
use ::cmd::Command as Cmd;

use std::path::{
    Path,
    PathBuf,
};

use std::process::{
    Command,
    Stdio,
};

use std::io::{
    BufWriter,
    Write,
};

#[derive(Debug)]
pub struct Hooks {
    post_change: Option<PathBuf>
}

fn get_hook_str<'a>(command: &'a Cmd) -> (&'a str, Option<&'a str>, Option<&'a str>) {
    match *command {
        Cmd::Init => ("init", None, None),
        Cmd::ListKeys => ("ls", None, None),
        Cmd::PutString(ref key, ref val) => ("put", Some(key.as_str()), Some(val.as_str())),
        Cmd::Drop(ref key) => ("drop", Some(key.as_str()), None),
        Cmd::CreateEmptyList(ref key) => ("emptyList", Some(key.as_str()), None),
        _ => unimplemented!()
    }
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

    pub fn run_post_change_hook(&self, result: &UiResult, command: &Cmd) -> Result<bool, String> {
        match self.post_change {
            Some(ref post_change_hook) => {

                let (action, key_opt, val_opt) = get_hook_str(command);

                let mut cmd_builder = Command::new(post_change_hook);
                cmd_builder.stdin(Stdio::piped());

                cmd_builder.arg(action);
                
                if let Some(key) = key_opt {
                    cmd_builder.arg(key);
                }

                if let Some(val) = val_opt {
                    cmd_builder.arg(val);
                }

                let mut hook_child = try!(cmd_builder.spawn().map_err(|e| e.to_string()));
                {
                    let stdin = try!(hook_child.stdin.as_mut().ok_or("no stdin".to_string()));
                    let mut stdin_buf = BufWriter::new(stdin);
                    try!(write!(stdin_buf, "{}", result).map_err(|e| e.to_string()));
                }
                let status = try!(hook_child.wait().map_err(|e| e.to_string()));
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
    use ::cmd::Command;

    #[test]
    fn test_load_from_dir() {
        let hooks = Hooks::load_from_dir("test/hooks");

        assert_eq!(Some(PathBuf::from("test/hooks/.postChange")), hooks.post_change);
    }

    #[test]
    fn test_run_post_change() {
        let hooks = Hooks::load_from_dir("test/hooks");

        let result = hooks.run_post_change_hook(&UiResult::Ok, &Command::Init);

        assert_eq!(Ok(true), result);
    }
}

