// The vectors could be pointers for easy updates
// to avoid having to reallocate the whole struct.
#[derive(Copy, Drop)]
struct PhysicalObj {
    id: felt,
    kind: felt,
    position: (felt, felt),
    velocity: (felt, felt),
}