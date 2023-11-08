use clap::Parser;
use std::error::Error;
use std::fs::read;
use std::io::Cursor;
use suppaftp::FtpStream;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub user_name: String,

    #[arg(short, long)]
    pub password: String,

    #[arg(short, long)]
    pub source_path: String,

    #[arg(short, long)]
    pub target_path: String,

    #[arg(short, long)]
    pub addr: String,
}

impl Cli {
    pub fn run() -> Result<(), Box<dyn Error>> {
        let Cli {
            user_name,
            password,
            source_path,
            target_path,
            addr,
        } = Cli::parse();

        let mut ftp_stream = FtpStream::connect(addr)?;
        ftp_stream.login(&user_name, &password)?;

        // entry target folder
        entry_directory(&mut ftp_stream, &target_path)?;

        // upload source file
        let file_name = extract_file_name_from_path(&source_path);
        let file = read(&source_path)?;
        let file_bytes = file.as_slice();
        let mut reader = Cursor::new(file_bytes);
        ftp_stream.put_file(&file_name, &mut reader)?;

        ftp_stream.quit()?;
        Ok(())
    }
}

// Entry the target path, if the target directory is not existed, than crate and entry it.
fn entry_directory(ftp: &mut FtpStream, target: &String) -> Result<(), Box<dyn Error>> {
    let target_vec: Vec<String> = target
        .split('/')
        .filter(|&x| !x.is_empty())
        .map(String::from)
        .collect();

    for path in target_vec.iter() {
        let current_list = ftp.nlst(None)?;

        if current_list.contains(path) {
            ftp.cwd(path)?;
        } else {
            ftp.mkdir(path)?;
            ftp.cwd(path)?;
        }
    }

    Ok(())
}

fn extract_file_name_from_path(source: &String) -> String {
    if let Some(index) = source.rfind('/') {
        String::from(&source[(index + 1)..])
    } else {
        source.clone()
    }
}
