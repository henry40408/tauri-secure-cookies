# tauri-secure-cookies

> I try to reproduce bug mentioned in [[MacOS / Linux] WebKit doesn’t set Secure cookies on localhost #444](https://github.com/tauri-apps/wry/issues/444)

## How to run

Setup [mkcert](https://github.com/FiloSottile/mkcert)

```shell
$ mkcert -install
```

Run simple HTTPS server

```shell
$ git clone https://github.com/henry40408/tauri-secure-cookies.git
$ cd tauri-secure-cookies/server
$ mkcert localhost # generates localhost.pem and localhost-key.pem
$ npm install
$ npm run dev # server is running on port 3000
```

Run wry application

```shell
$ git clone https://github.com/henry40408/tauri-secure-cookies.git
$ cd tauri-secure-cookies/app
$ cargo run # run wry application
```

## How to reproduce

1. Right click in webview of wry application and click "Reload"
2. The number should be increased every time the windows is reloaded
3. Close the wry application
4. Re-open wry application
5. On Windows, number is preseved as it is before windows is closed, but on Linux, number is reset to zero

## TODO

- [ ] Test on macOS

## License

MIT