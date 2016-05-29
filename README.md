# Rusticks
Rusticks is a barebones kernel written in Rust as an excercise in learning low level programming with Rust. It is more or less a Rust rewrite of [Sticks](https://github.com/balramr/sticks).

On booting, it shows a blank screen with some text on it. It has basic keyboard support and echos typed characters on to the screen.

## Building
Building Rusticks requires NASM, a Rust cross compiler and GNU ld. On a new Vagrant Ubuntu 14.04 VM, install the dependencies the following way:

```
curl https://sh.rustup.rs -sSf | sh
rustup target add i686-unknown-linux-gnu
sudo apt-get install nasm
sudo apt-get install qemu-system-x86
```

Use `make` to build and `make run` to run in QEMU.

Building on OS X should also work as long as GNU ld is available.

## Resources
Since Rusticks is based on Sticks, the information provided in the [Sticks readme](https://github.com/balramr/sticks) also applies here.

In addition, [rustboot](https://github.com/charliesome/rustboot) and [puddle](https://github.com/jvns/puddle) were useful references, despite being written before Rust 1.0. If starting from scratch, [Philipp Oppermann's guide](http://os.phil-opp.com) looks like an excellent resource.

## License
Rusticks is available in the public domain. See COPYING.