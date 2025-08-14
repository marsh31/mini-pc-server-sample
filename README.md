# README

MiniPCでサーバーを立てて見るサンプル。  
勉強用です。  


## debug

```make
make debug
```


## release

```make
make release
```


## install

```make
make install
```


## API

```bash
curl http://ミニPCのIP:3000/healthz
curl http://ミニPCのIP:3000/hello
curl -X POST http://ミニPCのIP:3000/echo \
  -H 'Content-Type: application/json' \
  -d '{"message":"hi"}'
```

