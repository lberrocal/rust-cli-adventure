# rust-cli-adventure
Console based game 

## Development

For development purposes in Windows, your .vscode files should look like this:

### launch.json
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'rust-cli-adventure'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=rust-cli-adventure",
                    "--package=rust-cli-adventure"
                ],
                "filter": {
                    "name": "rust-cli-adventure",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'rust-cli-adventure'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=rust-cli-adventure",
                    "--package=rust-cli-adventure"
                ],
                "filter": {
                    "name": "rust-cli-adventure",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```