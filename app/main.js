const {Webview}=require("webview-nodejs")
const wv=new Webview();
wv.size(800,600);
wv.navigate(`file://${__dirname}/src/index.html`)
wv.bind("runInNode",(w,s)=>{
    return eval(s);
})
wv.show();