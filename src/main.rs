#![allow(unused)]

use vessel::*;

use module::*;

mod module;
mod vessel;

fn main() {
    let pilot_cabin: Room<RoomTypeParams> = Room::from(RoomTypeParams::Cube { x: 5.0,
        y: 5.4,
        z: 1.9,
    });
    let reactor_room: Room<RoomTypeParams> = Room::from(RoomTypeParams::Sphere(3.0));
    let storage_room: Room<RoomTypeParams> = Room::from(RoomTypeParams::Cylinder { 
        h: 2.4,
        r: 3.7,
    });

print_params(pilot_cabin) 
}

fn print_params(room: Room<RoomTypeParams>) {
    println!("{:#?}", room);

}

