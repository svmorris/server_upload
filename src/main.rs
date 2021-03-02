use std::fs;
use uuid::Uuid;
use std::path::Path;
use std::{thread, time};
use std::process::Command;
use subprocess::Exec;


const TIMEOUT: u64        = 3000;
const SCAN_TIMEOUT: u64   = 1000;
const FOLDERPATH: &str    = "/tmp/upload_file/";
const SERVER_PATH: &str   = "chhon@butorhaz.hopto.org:/storage/http_files/";
const HOSTING_PATH: &str  = "https://butorhaz.hopto.org/files/";


fn main()
{
    loop
    {
        setup();
        let filename = get_file();
        println!("{}", filename);


        let filepath_static = FOLDERPATH.to_owned() + &*filename;


        let get_path: String = upload_file(&filepath_static);



        clipboard_push(get_path, filename);


        // sleep a few seconds
        thread::sleep(time::Duration::from_millis(TIMEOUT));
    }
}



fn get_file() -> String
{
    loop
    {
        //read dir
        let paths = fs::read_dir(FOLDERPATH).unwrap();

        for path in paths
        {
            // get filename
            let _filename = String::from(path.unwrap().file_name().into_string().unwrap());

            // create a static string and concat with folderpath
            let filepath_static = FOLDERPATH.to_owned() + &*_filename;

            // check if said path is a file
            if Path::new(&filepath_static).metadata().unwrap().is_file() == true
            {
                // make sure its not a part file
                if _filename.contains(".part") == false || _filename.contains(".temp.") == false || _filename.contains(".f137.") == false
                {
                    // return if file
                    return _filename;
                }
            }
        }

        // sleep a few seconds
        thread::sleep(time::Duration::from_millis(SCAN_TIMEOUT));
    }
}



fn setup()
{
    // if folder doesnt exist then creat the folder
    if Path::new(FOLDERPATH).exists() == false
    {
        println!("creating monitoring folder: {}", FOLDERPATH);

        Command::new("mkdir")
            .arg(FOLDERPATH)
            .output()
            .expect("Failed to execute command");
    }


    // if monitored path is not a folder then panic
    else if Path::new(FOLDERPATH).metadata().unwrap().is_dir() == false
    { 
        panic!("Monitored path is not a folder!");
    }


    // delete the first file in the folder
    else
    {
        let paths = fs::read_dir(FOLDERPATH).unwrap();
        for path in paths
        {
            // get filepath and name
            let filepath_static = FOLDERPATH.to_owned() + &*String::from(path.unwrap().file_name().into_string().unwrap());

            // delete the file
            Command::new("rm")
                .arg("-rf")
                .arg(filepath_static)
                .output()
                .expect("Failed to delete file!");

            return;
        }

    }
}



fn upload_file(filename: &str) -> String
{
    let mut extension = String::from("");

    if Path::new(filename).extension() != None
    {
        let t_extension = Path::new(filename)
            .extension()
            .unwrap()
            .to_str()
            .unwrap();

        extension.push('.');
        extension.push_str(t_extension);
    }

    let filename_uuid = Uuid::new_v4().to_string();

    Command::new("rsync")
        .arg("-avz")
        .arg("--progress")
        .arg(filename)
        .arg(SERVER_PATH.to_owned() + &filename_uuid + &extension)
        .output()
        .expect("could not upload file :(");


    return String::from(HOSTING_PATH.to_owned() + &filename_uuid + &extension);
}



fn clipboard_push(url: String, filename: String)
{
    // execute as shell command
    Exec::shell("echo '".to_owned() + &url + "' | xclip -sel clip")
        .join()
        .expect("failed to copy to clipboard");

    // run notification that file had been copied
    Command::new("notify-send")
        .arg("File: '".to_owned() + &filename + "' has been copied to clipboard!")
        .output()
        .expect("could not run notify-send");
}
