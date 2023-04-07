# Pyrolusite
My personal static blog generater written in Rust.
[<sup>Demo</sup>](https://mxcop.github.io/pyrolusite)

<br>

<h2>Post format <sup><code>.md</code></sup></h2>

```
{{ title }}
{{ day/month/year }}

{{ description }}

---

{{ markdown }}
```

## Commands
<sub>Build</sub>
```
$ pyro build [PATH]

Options:
  -o, --output <OUTPUT>  Build directory [default: ./build/]
  -s, --styles <STYLES>  Stylesheets directory, contents copied into the build directory if it exists [default: ./styles/]
```

<br>

## Building Example
This command will build all markdown files inside `./md` into `./docs`.
```
$ cargo run build ./md -o ./docs
```

<br>

<sub>© 2023 Max, All rights reserved.</sub>
