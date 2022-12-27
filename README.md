# StackPNG

A CLI tool built in Rust for creating Minecraft-style animations from a PNG sequence. It will stack the images, automatically resize them, and create a `.mcmeta` file for you.

## Installing

Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) installed.

Clone the git repository like this:

```command
git clone https://github.com/wowkster/StackPNG
cd StackPNG
```

Then compile and install with cargo:

```command
cargo install --path .
```

Now you can run the command anywhere and stack images to your heart's content:

```command
stackpng --help
```
