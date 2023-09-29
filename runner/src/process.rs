use std::process::Command;

use color_eyre::eyre::Error;

pub struct CargoProcess {
    pub name: String,
    pub release: bool,
    pub params: Vec<(String, String)>,
    pub child: Option<std::process::Child>,
}

impl CargoProcess {
    pub fn new(name: &str, release: bool, params: &[(&str, &str)]) -> Self {
        Self {
            name: name.to_string(),
            release,
            params: params
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            child: None,
        }
    }

    pub fn start(&mut self) -> Result<(), Error> {
        let mut cmd = Command::new("cargo");
        cmd.arg("run").arg("--bin").arg(self.name.as_str());
        if self.release {
            cmd.arg("--release");
        }
        if self.params.len() > 0 {
            cmd.arg("--");
            for (k, v) in &self.params {
                cmd.arg(format!("--{}", k));
                cmd.arg(v);
            }
        }

        let child = cmd.spawn()?;
        self.child = Some(child);
        Ok(())
    }

    pub fn is_running(&mut self) -> bool {
        if let Some(child) = self.child.as_mut() {
            match child.try_wait() {
                Ok(Some(_)) => false,
                Ok(None) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        if let Some(child) = self.child.as_mut() {
            child.kill()?;
        }
        Ok(())
    }
}

impl Drop for CargoProcess {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            child.kill().unwrap();
            child.wait().unwrap();
        }
    }
}
