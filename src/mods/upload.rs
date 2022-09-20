use ftp::FtpStream;
use std::fs::File;
//use std::io::Cursor;
use std::str;
use std::{path::Path, thread};

pub fn upload_file_ftp<P: AsRef<Path>>(
    url: &str,
    user: &str,
    password: &str,
    file_name: P,
) -> () {
    let url = url.to_string();
    let file_name = file_name.as_ref().to_owned();
    let user = user.to_owned();
    let password = password.to_owned();
    thread::spawn(move || {
        let mut ftp_stream = FtpStream::connect(url).unwrap();
        ftp_stream.login(&user, &password).unwrap();
        ftp_stream.cwd("speedtest").unwrap();
        let file = File::open(file_name).unwrap();
        ftp_stream.transfer_type(ftp::types::FileType::Binary).unwrap_or_default();
        ftp_stream.put("result.png", &mut &file).unwrap();
        ftp_stream.quit().unwrap_or_default();
    });
    ()
}
