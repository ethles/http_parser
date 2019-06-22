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
FORMDATA BOUNDARY : --------------------------930808077691525746483916
HEADER : [["Content-Type", "multipart/form-data; boundary=--------------------------930808077691525746483916"], ["User-Agent", "PostmanRuntime/7.15.0"], ["Accept", "*/*"], ["Cache-Control", "no-cache"], ["Postman-Token", "36afcac3-503d-4309-b2ab-2a7ccbc4163a"], ["Host", "localhost:8080"], ["accept-encoding", "gzip, deflate"], ["content-length", "564"], ["Connection", "keep-alive"]]
===PROCESS DATA===
FormData { name: "s", value: Text("\r\nsfddsf\r\n") }
FormData { name: "as", value: File(Stream, "temp.txt", [67, 111, 110, 116, 101, 110, 116, 45, 84, 121, 112, 101, 58, 32, 116, 101, 120, 116, 47, 112, 108, 97, 105, 110, 13, 10, 13, 10, 108, 115, 107, 100, 102, 106, 115, 108, 107, 100, 102, 106, 10, 115, 108, 107, 102, 106, 115, 100, 108, 102, 106, 107, 115, 10, 115, 107, 108, 100, 102, 106, 100, 107, 115, 102, 106, 10, 115, 100, 102, 107, 115, 100, 102, 106, 10]) }
FormData { name: "ass", value: File(Stream, "tempb", [67, 111, 110, 116, 101, 110, 116, 45, 84, 121, 112, 101, 58, 32, 97, 112, 112, 108, 105, 99, 97, 116, 105, 111, 110, 47, 111, 99, 116, 101, 116, 45, 115, 116, 114, 101, 97, 109, 13, 10, 13, 10, 108, 115, 107, 100, 102, 106, 115, 108, 107, 100, 102, 106, 10, 115, 108, 107, 102, 106, 115, 100, 108, 102, 106, 107, 115, 10, 115, 107, 108, 100, 102, 106, 100, 107, 115, 102, 106, 10, 115, 100, 102, 107, 115, 100, 102, 106, 10]) }
```

### Output File
```
╭─ethles@ETHLES ~ 
╰─$ tree  -s -h --du temp    
temp
├── [  75]  temp.txt
└── [  89]  tempb

  292 used in 0 directories, 2 files
```

## How To Use 

1. change output path in main.rs manually.
2. $ `cargo run`

## Update Plan

[-] no more update `:D`