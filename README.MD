# Warp template
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/tyan-boot/warp-tpl/Test%20Build?style=flat-square)
![GitHub](https://img.shields.io/github/license/tyan-boot/warp-tpl?style=flat-square)

`warp-tpl` is a simple template for rust [warp](https://github.com/seanmonstar/warp) web framework.

## Structure
```
├─app
│  │  Cargo.toml       
│  │
│  └─src
│      │  auth.rs
│      │  errors.rs
│      │  main.rs
│      │  state.rs
│      │  views.rs
│      │
│      └─views
│              index.rs
│
└─macros
    │  Cargo.toml
    │
    └─src
            lib.rs

```

* `macros` proc crate to convert `async fn xxx() -> Result<_, OtherError>` to `async fn xxx() -> Result<_, Rejection>` 
which is required in `warp`. Typically you could ignore this crate.
* `app` in which you write your web code.
* `app/src/auth.rs` example for app authorization.
* `app/src/errors.rs` contains user defined errors and code required to generate `warp::Reply`
* `app/src/state.rs` place app state like database or config used in route function in this file.
* `app/src/views.rs` register your api route in `build_filter`.
* `app/src/views/*.rs` app route goes here.
* `app/src/main.rs` entry of your app, init config, connect to database, etc.

## Usage
```shell script
git clone https://github.com/tyan-boot/warp-tpl
```

then write your code in `app`.

## Contribute
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

PRs and issues are welcome.