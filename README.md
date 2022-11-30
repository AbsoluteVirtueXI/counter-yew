```zsh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
git clone git@github.com:AbsoluteVirtueXI/counter-yew.git
cd counter-yew
trunk serve
```

Then see counter app on http://127.0.0.1:8080
