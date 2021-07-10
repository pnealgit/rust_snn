# rust_snn

Implementation by Phillip R. Neal
philn1984@gmail.com

Developed and tested on a clapped out
MacAir running macOS Catalina Verison 10.15.7

For this project, the goal is for the
rover to learn to keep away from the walls and
the square in the middle.

Some of the ideas are from :

Evolution of Spiking Neural Circuits in Autonomous Mobile Robots
Dario Floreano, Yann Epars, Jean-Christophe Zufferey, Claudio Mattiussi
INTERNATIONAL JOURNAL OF INTELLIGENT SYSTEMS, VOL. XX, 2005
(The SNN paper)

Spiking Nerural Networks with a ga are a lot cooler than
the usual Rumelhart-McClelland feed-forward neural networks.
It is a specific case of an extreme learning machine

Some of the Rust code with Nannou is from
https://github.com/nannou-org/nannou


I assume you have developed at least 1 program in Rust 
and you are familiar with the cargo toolset.


TO RUN:

Get to command line

cd to rust_snn directory

cargo build

This will take some time. 
Nannou has to be loaded and compiled.

Then

cargo run 



