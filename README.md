# Simple blockchain and mining in Rust


## create block in blockchain
```bash
curl -X POST http://localhost:8080 \
     -H "Content-Type: application/json" \
     -d '{"bpm":21}'
```

## get blockchain
```bash
curl localhost:8080 | jq
# or
curl localhost:8080 
```


## showcase

[![](./assets/screen_show.mp4)](./assets/screen_show.mp4)



### original implementation
[Code your own blockchain mining algorithm in Go!](https://mycoralhealth.medium.com/code-your-own-blockchain-mining-algorithm-in-go-82c6a71aba1f)