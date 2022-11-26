struct Vector {
    x: felt,
    y: felt,
}

// The vectors could be pointers for easy updates
// to avoid having to reallocate the whole struct.
struct PhysicalObj {
    type: felt,
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}