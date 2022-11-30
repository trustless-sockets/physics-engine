use physics::structs::PhysicalObj;

func newObj( id: felt, kind: felt, x: felt, y: felt ) -> PhysicalObj {
    PhysicalObj{
        id: id,
        kind: kind,
        position: (x, y),
        velocity: (0, 0),
    }
}

func applyForce( obj: PhysicalObj, force: (felt, felt) ) -> PhysicalObj {
    let (x, y) = obj.position;
    let (vx, vy) = obj.velocity;
    let (ax, ay) = force;

    PhysicalObj{
        id: obj.id,
        kind: obj.kind,
        position: (x, y),
        velocity: (vx + ax, vy + ay),
    }
}

func updateObject( obj: PhysicalObj ) -> PhysicalObj {
    let (x, y) = obj.position;
    let (vx, vy) = obj.velocity;

    PhysicalObj{
        id: obj.id,
        kind: obj.kind,
        position: (x + vx, y + vy),
        velocity: (vx, vy),
    }
}