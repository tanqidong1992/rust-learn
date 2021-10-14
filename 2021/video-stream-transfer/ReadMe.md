# IPC视频流转发测试项目
## 环境
1. Rust 1.54
2. VCPKG
3. Visual Studio Code.
4. LLVM,LLVM-Config.
4. ...
## 构建
```shell
cargo build
```
## 运行
```shell
./env.sh
cargo run
```
## 调试
配置
```json
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'app'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=app",
                    "--package=app"
                ],
                "filter": {
                    "name": "app",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}",
            "env": {
                "LD_LIBRARY_PATH": "${workspaceFolder}/dh/x64"
            }
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'app'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=app",
                    "--package=app"
                ],
                "filter": {
                    "name": "app",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in library 'dh'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--lib",
                    "--package=dh"
                ],
                "filter": {
                    "name": "dh",
                    "kind": "lib"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```