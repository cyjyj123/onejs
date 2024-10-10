extern crate msgbox;
use msgbox::IconType;
use std::{env::*, process::Command};
fn main() {
    let mut cmd=Command::new("node");
    let nodeVersionOutputResult=cmd.args(["--version",">log.txt"]).output();
    match nodeVersionOutputResult{
        Ok(nodeVersionOutput)=>{
            let nodeVersionResult=String::from_utf8(nodeVersionOutput.stdout);
            match nodeVersionResult{
                Ok(nodeVersion)=>{
                    // 版本比较和运行
                    println!("Start Running NodeJS");
                    let exePath=current_exe().unwrap();
                    let basePathArr:Vec<&str>=exePath.to_str().unwrap().split("/").collect();
                    let mut basePath=String::new();
                    for i in 0..basePathArr.len()-1{
                        basePath+=basePathArr[i];
                        if(i!=basePathArr.len()-2){
                            basePath+="/";
                        }
                    }
                    let nodeSrcRootPath=String::from(basePath+"/app/main.js");
                    println!("Run : {}",nodeSrcRootPath);
                    let mut cmd2=Command::new("node");
                    cmd2.args([nodeSrcRootPath]).output().expect("failed");
                },
                Err(errMsg)=>{
                    msgbox::create("Tips","You need install NodeJS Runtime",IconType::Info);
                }
            }
        },
        Err(errMsg)=>{
            msgbox::create("Tips","You need install NodeJS Runtime",IconType::Info);
        }
    }
}
