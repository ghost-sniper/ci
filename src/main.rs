mod execute;

use std::path::{Path, PathBuf};
use dirs::audio_dir;
use git2::build::{RemoteCreate, RepoBuilder};
use git2::{Error, ErrorCode, FetchOptions, Remote, RemoteCallbacks, RemoteConnection, Repository};
use url::Url;

fn main() {
    // let mut url = url::Url::parse("http://106.54.169.19:3000/yang/perceptor-server.git").unwrap();
    // let password = url.password().unwrap();
    // println!("password:{}", password);
    // let username = url.username();
    // println!("username:{}", username);
    // url.set_username("tony").expect("Add username Failed");
    // url.set_password(Some("QPMZqpmz1234!")).expect("Add Password Failed");
    // println!("url: {}",url.as_str());

    // let home_dir = dirs::home_dir().unwrap();
    // println!("Current user's home dir is {}", home_dir.as_os_str().to_str().unwrap());
    // let work_dir = home_dir.as_path().join("repository");
    // println!("Work Dir {}", work_dir.to_str().unwrap());
    //let remote = Remote::create_detached("http://106.54.164.19:3000/yang/perceptor-server.git").expect("Get Remote Failed");
    //let repo = Repository::clone("http://106.54.164.19:3000/yang/perceptor-server.git",Path::new("/home/tony/repo")).unwrap();
    // let mut remote_callbacks = RemoteCallbacks::new();
    // let mut fetch_options = FetchOptions::new();
    // fetch_options.remote_callbacks(remote_callbacks);
    //let mut repo_builder = RepoBuilder::new();
    // repo_builder.fetch_options(fetch_options).clone("");

    let home_dir = get_home_dir();
    let repository_dir = home_dir.join("repo");
    //println!("Repository Directory: {}",repository_dir.to_str().unwrap());
    let mut url = Url::parse("http://106.54.164.19:3000/yang/perceptor-server.git").unwrap();
    let mut segments = url.path_segments().ok_or_else(||"cannot be base").unwrap();
    println!("{}",segments.next().unwrap());
    println!("{}",segments.next().unwrap());
    // let mut remote_callbacks = RemoteCallbacks::new();
    // let mut fetch_options = FetchOptions::new();
    // fetch_options.remote_callbacks(remote_callbacks);
    //
    // let mut repo_builder = RepoBuilder::new();
    // repo_builder.fetch_options(fetch_options);
    // match repo_builder.clone(url.as_str(), Path::new(repository_dir.as_path())) {
    //     Ok(_) => {}
    //     Err(err) => {
    //         if err.code() == ErrorCode::Auth {
    //             url.set_username("tony").unwrap();
    //             url.set_password(Some("QPMZqpmz1234!")).unwrap()
    //         }
    //     }
    // }
}

// pub fn has_username(url: &Url) -> bool {
//     return if url.username().is_empty() {
//         false
//     } else {
//         true
//     };
// }
//
// pub fn has_password(url: &Url) -> bool {
//     return match url.password() {
//         Some(passwd) => true,
//         None => false,
//     };
// }

pub fn check_local_repository(folder_name: String) {
    let path = Path::new("/home/tony/repository");
}

pub fn get_home_dir() -> PathBuf {
    return match dirs::home_dir() {
        Some(dir) => dir,
        None => {
            let path_buf = PathBuf::new();
            path_buf.join("/")
        }
    };
}

pub fn get_remote_repository_name(url: Url) -> String{
    let url_str = url.as_str();
    let rg = regex::Regex::new("");
}