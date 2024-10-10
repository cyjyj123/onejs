window.onload=()=>{};

const handleClick=async ()=>{
    document.write("test");
    //let im=await runInNode(`const fs=require("fs");`);
    let resp=await runInNode(`require("fs").readFileSync(__dirname+"/package.json").toString()`);
    document.write(resp)
}