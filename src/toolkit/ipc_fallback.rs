use std::io::{Error, ErrorKind};
use std::sync::Mutex;

static MOCK_CHANNELS: std::sync::OnceLock<Mutex<std::collections::HashMap<String, String>>> =
    std::sync::OnceLock::new();

fn get_channels() -> &'static Mutex<std::collections::HashMap<String, String>> {
    MOCK_CHANNELS.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

pub struct FallbackIpcServer {
    name: String,
}

impl FallbackIpcServer {
    pub fn bind(name: &str) -> Result<Self, Error> {
        Ok(Self {
            name: name.to_string(),
        })
    }

    pub fn accept_and_respond<F>(&self, handler: F) -> Result<(), Error>
    where
        F: Fn(&str) -> String,
    {
        // Wait up to 2 seconds for a request
        for _ in 0..200 {
            {
                let mut channels = get_channels()
                    .lock()
                    .map_err(|_| Error::new(ErrorKind::Other, "Lock poisoned"))?;
                if let Some(msg) = channels.remove(&self.name) {
                    let response = handler(&msg);
                    channels.insert(format!("{}_resp", self.name), response);
                    return Ok(());
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        Err(Error::new(ErrorKind::TimedOut, "Mock IPC read timeout"))
    }
}

pub struct FallbackIpcClient {
    name: String,
}

impl FallbackIpcClient {
    pub fn connect(name: &str) -> Result<Self, Error> {
        Ok(Self {
            name: name.to_string(),
        })
    }

    pub fn send_request(&mut self, msg: &str) -> Result<String, Error> {
        let mut channels = get_channels()
            .lock()
            .map_err(|_| Error::new(ErrorKind::Other, "Lock poisoned"))?;
        channels.insert(self.name.clone(), msg.to_string());
        drop(channels);

        for _ in 0..200 {
            std::thread::sleep(std::time::Duration::from_millis(10));
            let mut channels = get_channels()
                .lock()
                .map_err(|_| Error::new(ErrorKind::Other, "Lock poisoned"))?;
            if let Some(resp) = channels.remove(&format!("{}_resp", self.name)) {
                return Ok(resp);
            }
        }
        Err(Error::new(ErrorKind::TimedOut, "Mock IPC timeout waiting for response"))
    }
}
