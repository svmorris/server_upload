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
notify-send
```
other:
* have a webserver that it can upload to
* use autologin with ssh
* have a computer to use it on


### insall
```
cargo build --release
sudo cp /target/release/server_upload /usr/local/bin/
```

### usage

you can start monitoring by running `server_upload &`

recomended would be to use something like rofi that will keep the process running in the background

you could also add it to your crontab
```
@reboot server_upload
```


#### warning
the program deletes the contents of the monitored folder at startup, so you need to start it before copying files over
