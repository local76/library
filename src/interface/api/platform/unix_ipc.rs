use std::io::{Error, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

pub struct UnixIpcServer {
    listener: UnixListener,
    socket_path: PathBuf,
}

impl UnixIpcServer {
    pub fn bind(name: &str) -> Result<Self, Error> {
        let socket_path = std::env::temp_dir().join(format!("{}.sock", name));
        if socket_path.exists() {
            let _ = std::fs::remove_file(&socket_path);
        }

        let listener = UnixListener::bind(&socket_path)?;
        Ok(Self {
            listener,
            socket_path,
        })
    }

    pub fn accept_and_respond<F>(&self, handler: F) -> Result<(), Error>
    where
        F: Fn(&str) -> String,
    {
        let (mut stream, _) = self.listener.accept()?;
        let mut buffer = [0u8; 65536];
        let n = stream.read(&mut buffer)?;
        if n > 0 {
            let req_str = String::from_utf8_lossy(&buffer[..n]);
            let response = handler(req_str.trim_end_matches('\0'));
            stream.write_all(response.as_bytes())?;
        }
        Ok(())
    }
}

impl Drop for UnixIpcServer {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.socket_path);
    }
}

pub struct UnixIpcClient {
    stream: UnixStream,
}

impl UnixIpcClient {
    pub fn connect(name: &str) -> Result<Self, Error> {
        let socket_path = std::env::temp_dir().join(format!("{}.sock", name));
        let stream = UnixStream::connect(socket_path)?;
        Ok(Self { stream })
    }

    pub fn send_request(&mut self, msg: &str) -> Result<String, Error> {
        self.stream.write_all(msg.as_bytes())?;
        let mut buffer = [0u8; 65536];
        let n = self.stream.read(&mut buffer)?;
        let resp_str = String::from_utf8_lossy(&buffer[..n]).into_owned();
        Ok(resp_str)
    }
}
