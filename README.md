# rust-wasm-roguelike
A Rust learning project viewable at [https://elg.gg/roguelike/](https://elg.gg/roguelike/)

This is a bare-bones traditional roguelike engine written in Rust and compiled to WebAssembly. 
It uses the [bresenham](https://docs.rs/bresenham/) and [rand](https://docs.rs/rand/) crates for game logic.
All other dependencies are for WebAssembly or JavaScript interaction.


## Dungeon generation
The dungeon is randomly generated, using my own implementation of
[basic binary space partitioning dungeon generation](http://www.roguebasin.com/index.php/Basic_BSP_Dungeon_generation).

## Canvas
The canvas module was initially intended to print to the console.
It was written before any idea of what the final project will be and therefore contains some functionality that remains unused.

The JS/HTML rendering method is not the most efficient (divs),
but the purpose of the project was more to have somewhere to implement Rust features on my own as I read through [the book](https://doc.rust-lang.org/book/).
