# rust-cli-adventure
Console based game 

## Development

For development purposes in Windows, your .vscode files should look like this:

### launch.json
```json
{
    "version": "0.2.0",
    "configurations": [{
        "name": "(Windows) Launch",
        "type": "cppvsdbg",
        "request": "launch",
        "program": "${workspaceRoot}/target/debug/rust-cli-adventure.exe",
        "externalConsole": true,
        "stopAtEntry": false,
        "cwd": "${workspaceRoot}",
        "preLaunchTask": "build-cli"
    }] 
    
}
```

### tasks.json
```json
{
    "version": "2.0.0",
    "tasks": [
		{
			"type": "cargo",
			"command": "build",
			"problemMatcher": [
				"$rustc"
			],
			"group": {
				"kind": "build",
				"isDefault": true
			},
			"label": "build-cli"
		}
	]
}
```