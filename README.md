# Internship task
## Effective File Transfer Between Machines for Remote Development

### Approach

The task assignment included a note advising us to avoid external libraries if possible. Therefore, I am making requests using a TCP stream from the standard library. The only external library I am using is for SHA hash calculation. There are probably ways to avoid this, but I believe this is a good solution rather than implementing it from scratch or trying to obtain the SHA hash using command execution in the running environment.

### Running the Server

```sh
python3 server/buggy_server.py
```

###  Running the Client
```sh
cargo run
```



