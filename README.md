[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)

# Moni
Moni is simple file monitor.  

![default](https://user-images.githubusercontent.com/43775946/53548534-0a357a00-3b75-11e9-9223-84d38bf0bc3d.PNG)

## Usage
- Server run  
The file information is output to the Log file and the terminal

`$ ./moni -l`

- Client run  

`$ ./moni`

> You can specify multiple file paths with arguments specified.  
 `./moni [FolderPath] [FolderPath] ..`  
> Otherwise it reads from 'watch_list' file.


## Install and Build
1. Clone the source with git
```
$ git clone git@githubcom:various3211workd/file_monitor_learning.git
```

2. Build  
```
$ cd moni
$ cargo build
```

