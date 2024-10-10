extern crate msgbox;
use msgbox::IconType;
use std::{env::*, process::Command,fs};
use std::path::Path;
use asar::AsarReader;
fn main() {
    let mut runMode="nopack"; // "nopack"(mode 1),"asar"(mode 2),default is nopack

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

                    // 确定运行模式，若有app.asar则更改为相应的运行模式，否则按照默认的nopack模式运行，不进行处理
                    if(Path::new((basePath.clone()+"/app.asar").as_str()).exists()){
                        runMode="asar";
                    }

                    if(runMode=="asar"){
                        // 将app.asar解压至当前目录（即basePath）的.app目录下
                        // 如果.app目录已存在，当作已经解压不处理
                        let bp=basePath.clone()+"/.app";
                        let asarExtPath=Path::new(bp.as_str());
                        if(!asarExtPath.exists()){
                            // 解压
                            fs::create_dir(asarExtPath);
                            let appAsarFile=fs::read(basePath.clone()+"/app.asar").unwrap();
                            let asarFileReader=AsarReader::new(&appAsarFile,None).unwrap();
                            // 创建目录
                            for file in asarFileReader.directories().keys(){
                                let extDirPathStr=bp.clone()+"/"+file.to_str().unwrap();
                                let extDirPath=Path::new(extDirPathStr.as_str());
                                if(!extDirPath.exists()){
                                    fs::create_dir(extDirPath);
                                }
                            }
                            // 创建文件
                            for file in asarFileReader.files().keys(){
                                let extFilePathStr=bp.clone()+"/"+file.to_str().unwrap();
                                let extFilePath=Path::new(extFilePathStr.as_str());
                                if(!extFilePath.exists()){
                                    // 将file复制到extFilePath
                                    let fileData=asarFileReader.read(file).unwrap().data();
                                    fs::write(extFilePath,fileData);
                                }
                            }
                            //asarFileReader.
                            //return;
                        }
                    }

                    let mut appMenu="/app";
                    if(runMode=="asar"){
                        appMenu="/.app";
                    }
                    let nodeSrcRootPath=String::from(basePath+appMenu+"/main.js");
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
