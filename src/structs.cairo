use physics::test_fn;

struct Vector {
    x: felt,
    y: felt,
}

// The vectors could be pointers for easy updates
// to avoid having to reallocate the whole struct.
struct PhysicalObj {
    obj_type: felt,
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

func test() -> felt {
    test_fn::yafn()
    // bleh()
}