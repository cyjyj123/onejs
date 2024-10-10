# OneJs
An open source lib that pack ECMAScript Application.
## Requirements
``` 
Rust // Only for Compile
NodeJS 
System WebView
// crates
msgbox // author: Jang Ryeol, license: MIT
asar // author: Lucy <lucy@absolucy.moe>, licenses: Apache-2.0 or MIT
webview-nodejs // author: Winterreisender, license: MIT
``` 
## Mode
### Mode 1
Version 1.0.0 only support Mode 1.
The project struct of this mode likes:
```
/
    /app
    -main
```
### Mode 2
```
/
    -.app
    -app.asar
    -main
```
## Attention
You should not save user data in `app`,`.app` ,`app.asar`.
## Usage
After git clone,
```
cd app_v1/src
```
and Use your code instead of it. Then,
```
cd ..
asar pack . ../app.asar
cd ..
cargo build
cp target/debug/onejs .
``` 
Then, rename onejs.
Finally , for an basic application, you only need `app.asar` ,`onejs`. 
## Update
### Version 2
Rename app to app_v1
Add Mode 2

## LICENSE
MIT