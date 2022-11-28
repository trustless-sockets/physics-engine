use physics::structs::Vector;
use physics::structs::PhysicalObj;

func addPhysicalObject( obj: PhysicalObj, lastObj: PhysicalObj ) -> felt {
    // Adds a object to array
    // Returns pointer to the object
    // Separate array to hold acceleration for the physical object
    // Returns the pointer to newly allocated array
}

func updatePhysicalObject( object: PhysicalObj, acceleration: Vector ) -> PhysicalObj {
    
    let newVel = Vector{
        x: object.velocity.x + acceleration.x,
        y: object.velocity.y + acceleration.y,
    };
    let newPos = Vector{
        x: object.position.x + object.velocity.x,
        y: object.position.y + object.velocity.y,
    };
    // Create PhysicalObj with newPos and newVel and return
    PhysicalObj{
        kind: 0,
        position: newPos,
        velocity: newVel,
    }
}

func main() {
    
}