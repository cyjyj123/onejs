window.onload=()=>{};

const handleClick=async ()=>{
    document.write("test");
    //let im=await runInNode(`const fs=require("fs");`);
    let resp=await runInNode(`require("fs").readFileSync("./app/package.json").toString()`);
    document.write(resp)
}