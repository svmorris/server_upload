filesize limits are fucked, esp on discord



what this program does:
* monitors a folder
* if there is a new file in the folder it uploads it to the specified server
* it creates a link to that, and copies it to your clipboard
* notify's you that its done
* deletes the file


setup and usage
===============
### dependencies
programs:
```
cargo
rsync
xclip
notify-send
```
other:
* have a webserver that it can upload to
* use autologin with ssh
* have a computer to use it on

NOTE for mac users:
* replace xclip with [xclip-for-mac](https://github.com/siers/xclip-for-mac), and alias/link it to xclip
* use the notify-send script included with the repo

### setup
you need to edit some variables in `src/main.rs`

* if multiple files are copied in the folder, TIMOUT sets the time from adding the link of the first file to the clipboard to starting work on the next one
```
const TIMEOUT: u64        = 3000;
```

* SCAN_TIMEOUT sets the interval of the program checking for new files
* make this larger if the program uses too much cpu
```
const SCAN_TIMEOUT: u64   = 1000;
```

* FOLDERPATH sets the monitored folder
```
const FOLDERPATH: &str    = "/tmp/upload_file/";
```

* SERVER_PATH is where rsync will upload the file
```
const SERVER_PATH: &str   = "chhon@butorhaz.hopto.org:/storage/http_files/";
```
* HOSTING_PATH is a link to the server hosting the file
* this should be a webserver that has access to the folder SERVER_PATH
```
const HOSTING_PATH: &str  = "https://butorhaz.hopto.org/files/";
```

* if any string from BLACKLIST found in the name of a file than the program will ignore it
* this is useful for .part and temp files that are used by programs while downloading
* **NOTE**: when modifying it make sure to change the array length `[&str; 3]` to reflect changes
```
const BLACKLIST:[&str; 3] = [".part", ".temp", ".f127."];
```



### insall
for linx:
```
cargo build --release
sudo cp target/release/server_upload /usr/local/bin/
```

for mac:
```
cargo build --release
sudo cp target/release/server_upload /usr/local/bin/
sudo cp other/notify-send /usr/local/bin/
sudo chmod o+x /usr/local/bin/notify-send

install xclip-for-mac and alias/link it to to xclip
```

for windows:
* im sorry
* im sure there are other cool programs for you elsewhere


### usage

you can start monitoring by running `server_upload &`

recomended would be to use something like `rofi` that will keep the process running in the background

you could also add it to your crontab
```
@reboot server_upload
```

#### command line arguments
`-k` -- kill program after one upload


#### warning
the program deletes the contents of the monitored folder at startup, so you need to start it before copying files over
