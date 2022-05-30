const http = require('http');
const port = 80
http.createServer((req,res)=>{
    console.log(req.url)
    res.writeHead(200, { 'Date': (new Date(Date.now()-1000*60)).toUTCString() });
    res.end(",");
}).listen(port, ()=>{console.log(`server is ruuning at ${port}`)})