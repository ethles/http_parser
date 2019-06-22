# http_parser

a http parser.

## HTTP Request Example
### Request struct
```
POST / HTTP/1.1
Content-Type: multipart/form-data; boundary=--------------------------080905360496688834795009
User-Agent: PostmanRuntime/7.15.0
Accept: */*
Cache-Control: no-cache
Postman-Token: 6020e13b-8475-42e7-86ef-3dd30fcc2f75
Host: localhost:8080
accept-encoding: gzip, deflate
content-length: 559
Connection: keep-alive

----------------------------080905360496688834795009
Content-Disposition: form-data; name="s"

sfddsf
----------------------------080905360496688834795009
Content-Disposition: form-data; name="as"; filename="temp.txt"
Content-Type: text/plain

lskdfjslkdfj
slkfjsdlfjks
skldfjdksfj
sdfksdfj

----------------------------080905360496688834795009
Content-Disposition: form-data; name="ass"; filename="tempb"
Content-Type: application/octet-stream

lskdfjslkdfj
slkfjsdlfjks
skldfjdksfj
sdfksdfj

----------------------------080905360496688834795009--
```
### Output Log
```
===PROCESS REQUEST LINE===
METHOD : POST
===PROCESS HEADER===
FORMDATA BOUNDARY : --------------------------496900655673395966503899
HEADER : [["Content-Type", "multipart/form-data; boundary=--------------------------496900655673395966503899"], ["User-Agent", "PostmanRuntime/7.15.0"], ["Accept", "*/*"], ["Cache-Control", "no-cache"], ["Postman-Token", "2d3d5feb-adf5-4400-b99b-a5acb81be22c"], ["Host", "localhost:8080"], ["accept-encoding", "gzip, deflate"], ["content-length", "564"], ["Connection", "keep-alive"]]
===PROCESS DATA===
FormData { name: "s", value: Text("sfddsf") }
FormData { name: "as", value: File(Stream, "temp.txt", [13, 10, 108, 115, 107, 100, 102, 106, 115, 108, 107, 100, 102, 106, 10, 115, 108, 107, 102, 106, 115, 100, 108, 102, 106, 107, 115, 10, 115, 107, 108, 100, 102, 106, 100, 107, 115, 102, 106, 10, 115, 100, 102, 107, 115, 100, 102, 106, 10]) }
FormData { name: "ass", value: File(Stream, "tempb", [13, 10, 108, 115, 107, 100, 102, 106, 115, 108, 107, 100, 102, 106, 10, 115, 108, 107, 102, 106, 115, 100, 108, 102, 106, 107, 115, 10, 115, 107, 108, 100, 102, 106, 100, 107, 115, 102, 106, 10, 115, 100, 102, 107, 115, 100, 102, 106, 10]) }
```

### Output File
```
╭─ethles@ETHLES ~ 
╰─$ tree  -s -h --du temp                                          
temp
├── [  49]  temp.txt
└── [  49]  tempb

  226 used in 0 directories, 2 files
```

## How To Use 

1. change output path in main.rs manually.
2. $ `cargo run`

## Update Plan

[-] no more update `:D`