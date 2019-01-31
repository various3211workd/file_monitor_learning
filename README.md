# File_Monitor_Learning

## Usage
- Server run  
The file information is output to the Log file and the terminal

`$ cargo run`  
or  
`$ ./moni -l <port>`

- Client run  

`$ cargo run`

> You can specify multiple file paths with arguments specified.  
 `cargo run [FolderPath] [FolderPath] ..`  
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

