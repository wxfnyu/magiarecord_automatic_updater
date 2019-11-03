extern crate reqwest;
extern crate zip;

use std::io;
use std::fs;
use std::fs::File;
use std::process::Command;
use scraper::{Html, Selector};
use std::env;
use std::path::Path;



fn main() {
    let args: Vec<String> = env::args().collect();
    let na_flag = args.iter().any(|x| x == "-NA" || x == "-na");
    let force_local_adb_flag = args.iter().any(|x| x == "-forceLocalADB");
    let force_adb_download_flag = args.iter().any(|x| x == "-forceADBDownload");
    let use_old_apk_flag = args.iter().any(|x| x == "-noAPKDownload");
    let no_install_flag = args.iter().any(|x| x == "-noInstall");

    if(force_adb_download_flag || ((!detect_native_adb() || force_local_adb_flag)  && !detect_prev_downloaded_adb())){
        //print!("Downloading ADB");
        //todo, request confirmation of acceptance of adb licence
        get_platform_tools();
    }
    
    if(!use_old_apk_flag){
        if(na_flag){
            print!("Using NA APK\n");
            get_na_apk();
        } else {
            print!("Using JP APK\n");
            get_jp_apk();
        }
    }

    let phonepath = "/data/local/tmp/magiarecord.apk";

    if(!no_install_flag){
        install_apk((force_local_adb_flag || !detect_native_adb()), phonepath);
    }
    
    print!("Done!")
} 

fn detect_prev_downloaded_adb() -> bool {
    let path_to_check = if cfg!(windows){
        ".\\platform-tools\\adb.exe"
    } else {
        "./platform-tools/adb"
    };
    return Path::new(path_to_check).exists();
}



fn detect_native_adb() -> bool {
    let res = match which::which("adb"){
        Ok(_) => true,
        Err(_) => false
    };
    return res;
}

fn get_platform_tools(){
    //by using this you're accepting the licence agreement for the android platform tools:
    //see agreement here: https://developer.android.com/studio/releases/platform-tools
    //this entire block is hacky i should really use a native adb server emulation instead
    //but that does not exist, so instead this will have to do
    let target = if cfg!(target_os="windows"){ //test windows
        "https://dl.google.com/android/repository/platform-tools-latest-windows.zip"
    } else if cfg!(target_os="macos"){ //test mac
        "https://dl.google.com/android/repository/platform-tools-latest-darwin.zip"
    } else if cfg!(target_os="linux"){ //test linux
        "https://dl.google.com/android/repository/platform-tools-latest-linux.zip"
    } else {
        "unknown"
    };
    if(target == "unknown"){
        print!("Not a known os, you should manually configure ADB for your platform before running this script");
        std::process::exit(1);
    }
    let mut resp = reqwest::get(target)
        .expect("request failed");
    
    let mut out = File::create("platformtools.zip")
        .expect("failed to create file (platformtools)");
    
    let _fileoutresult = io::copy(&mut resp, &mut out)
        .expect("Failed to ADB from remote to local file");
    //release the filehandle
    drop(out);

    unzip_archive("platformtools.zip");
    
}

//citation: https://github.com/mvdnes/zip-rs/blob/master/examples/extract.rs
//under MIT Licence
fn unzip_archive(path: &str){ 
    let file = fs::File::open(path).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    
    for i in 0 .. archive.len() {
        let mut file_to_extract = archive.by_index(i).unwrap();
        let outpath = file_to_extract.sanitized_name();

        if(file_to_extract.name().ends_with('/')){
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            let _ = io::copy(&mut file_to_extract, &mut outfile);
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file_to_extract.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    
}

fn process_apk_webpage(webpage: &str ) -> std::string::String {
    //parse shit here
    let document = Html::parse_document(webpage);
    let sel = Selector::parse("#download_link").unwrap();

    let res = String::from(document.select(&sel).next().unwrap().value().attr("href").unwrap());

    return res;
}

fn get_apk(target: &str){
    let mut resp = reqwest::get(target)
        .expect("request failed");
    let resp_text = resp.text().unwrap();
    let apk_url = process_apk_webpage(resp_text.as_str()); 
    resp = reqwest::get(apk_url.as_str())
        .expect("request failed");
    let mut out = File::create("magiarecord.apk")
        .expect("failed to create file (apk)");
    let _fileoutresult = io::copy(&mut resp, &mut out);
    //release the filehandle
    drop(out);
}

fn get_na_apk(){
    let target = "https://apkpure.com/magia-record-english/com.aniplex.magireco.en/download";
    get_apk(target);
}

fn get_jp_apk(){
    let target = "https://apkpure.com/%E3%83%9E%E3%82%AE%E3%82%A2%E3%83%AC%E3%82%B3%E3%83%BC%E3%83%89-%E9%AD%94%E6%B3%95%E5%B0%91%E5%A5%B3%E3%81%BE%E3%81%A9%E3%81%8B%E3%83%9E%E3%82%AE%E3%82%AB%E5%A4%96%E4%BC%9D/com.aniplex.magireco/download";
    get_apk(target);
}

//TODO better fail state detection
fn install_apk(local: bool, phonepath: &str){
    let adbcmd = if !local {
        "adb"
    } else if cfg!(windows){
        ".\\platform-tools\\adb.exe"
    } else {
        "./platform-tools/adb"
    };

    print!("Waiting for Android device on ADB\n");
    let mut adb_wait_process = Command::new(adbcmd)
        .arg("wait-for-device")
        .spawn()
        .expect("Failed to wait for a device on adb");
    print!("done!\nPushing APK\n");
    adb_wait_process.wait().expect("Failed while waiting for device");
    let mut adb_push_process = Command::new(adbcmd)
        .args(&["push", "magiarecord.apk", phonepath])
        .spawn()
        .expect("Failed to push apk");
    adb_push_process.wait().expect("Failed while waiting for apk push");
    print!("done!\nInstalling APK\n");
    let mut adb_install_process = Command::new(adbcmd)
        .args(&["shell", "pm", "install", "-i", "\"com.android.vending\"", "-r", phonepath])
        .spawn()
        .expect("Failed to install apk");
    adb_install_process.wait().expect("Failed while waiting on install");
}
