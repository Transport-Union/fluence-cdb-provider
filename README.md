# debuggin 

When you remove wasm import from tu_cdb_provider/modules/index/src/index.rs it builds without errors. 

Error: Embedded by rust-sdk metadata couldn't be parsed by serde: Error("trailing characters", line: 1, column: 187)

Caused by:
    trailing characters at line 1 column 187
Making sure all modules are downloaded and built... !
    Error: process exited with code 1
        Blocking waiting for file lock on package cache
        Blocking waiting for file lock on package cache
        Blocking waiting for file lock on package cache
        Finished release [optimized] target(s) in 0.23s
    Error: Embedded by rust-sdk metadata couldn't be parsed by serde: Error("trailing characters", 
    line: 1, column: 187)

    Caused by:
        trailing characters at line 1 column 187
    Debug info:
    /home/joera/.fluence/cargo/marine/0.12.6/bin/marine build --release
