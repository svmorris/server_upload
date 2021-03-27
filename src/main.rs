use std::fs;
use std::env;
use uuid::Uuid;
use std::process;
use std::path::Path;
use std::{thread, time};
use std::process::Command;
use subprocess::Exec;



const TIMEOUT: u64        = 3000;
const SCAN_TIMEOUT: u64   = 1000;
const FOLDERPATH: &str    = "/tmp/upload_file/";
const SERVER_PATH: &str   = "chhon@butorhaz.hopto.org:/storage/http_files/";
const HOSTING_PATH: &str  = "https://butorhaz.hopto.org/files/";
const UPLOAD_SUFFIX: &str = ".__uploading__";
const BLACKLIST:[&str; 7] = [UPLOAD_SUFFIX, ".temp", ".f127.", ".f299.", ".DS_Store", ".part", ".m4a"];
const NOTIFICATION_NAME: &str    = "SERVER-UPLOADER";


fn main()
{


    // run main loop
    loop
    {
        // setup checks if the folder is present and functional, as well as attempts to fix issues
        setup();

        // get filename, and full path
        let filename = &get_file();
        let filepath_static = FOLDERPATH.to_owned() + &filename;


        // upload file and get url
        let get_path: String = upload_file(&filepath_static);


        // put url on clipboard
        clipboard_push(get_path, filename);


        // delete uploaded file
        Command::new("rm")
            .arg("-rf")
            .arg(filepath_static)
            .output()
            .expect("Failed to delete file!");


        // get command line arguments
        let args: Vec<String> = env::args().collect();
        // if the second argument is -k then kill the program after one run
        if (args.len() == 2 ) && (args[1] == "-k")
        {
            process::exit(0x0);
        }

        // sleep a few seconds
        thread::sleep(time::Duration::from_millis(TIMEOUT));
    }
}


fn setup()
{
    // if folder doesnt exist then create the folder
    if Path::new(FOLDERPATH).exists() == false
    {
        println!("creating monitoring folder: {}", FOLDERPATH);
        Command::new("mkdir")
            .arg(FOLDERPATH)
            .output()
            .expect("Failed to execute command");


        Command::new("notify-send")
            .arg(NOTIFICATION_NAME)
            .arg("Created monitoring folder: ".to_owned() + FOLDERPATH)
            .output()
            .expect("failed to send notification");
    }


    // if monitored path is not a folder then panic
    else if Path::new(FOLDERPATH).metadata().unwrap().is_dir() == false
    {
        Command::new("notify-send")
            .arg(NOTIFICATION_NAME)
            .arg("FAILED: monitored path already exists, and is not a directory")
            .output()
            .expect("failed to send notification");

        panic!("Monitored path is not a folder!");
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
                let mut valid = true;

                for item in BLACKLIST.iter()
                {
                    println!("{} == {} : {:?}", item, _filename, !valid);
                    if _filename.contains(item)
                    {
                        valid = false;
                    }
                }


                if valid
                {
                    // create indicator file that the program has started uploading
                    println!("creating temp file: ");
                    Command::new("touch")
                        .arg(FOLDERPATH.to_owned() + &_filename + UPLOAD_SUFFIX)
                        .output()
                        .expect("Could not create file");

                    return _filename;
                }
            }
        }

        // sleep a few seconds
        thread::sleep(time::Duration::from_millis(SCAN_TIMEOUT));
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


    Command::new("rm")
        .arg("-rf")
        .arg(filename.to_owned() + UPLOAD_SUFFIX)
        .output()
        .expect("failed to delete temp file");


    return String::from(HOSTING_PATH.to_owned() + &filename_uuid + &extension);
}


fn clipboard_push(url: String, filename: &str)
{
    // execute as shell command
    Exec::shell("echo '".to_owned() + &url + "' | xclip -sel clip")
        .join()
        .expect("failed to copy to clipboard");

    // run notification that file had been copied
    Command::new("notify-send")
        .arg(NOTIFICATION_NAME)
        .arg("File: '".to_owned() + filename + "' has been copied to clipboard!")
        .output()
        .expect("could not run notify-send");
}
