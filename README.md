# Moni
Moni is simple file monitor.  


## Usage
- Server run  
The file information is output to the Log file and the terminal

`$ ./moni -l <port>`

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

