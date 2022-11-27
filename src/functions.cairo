use src::structs::Vector;
use src::structs::PhysicalObj;

func addPhysicalObject( obj: PhysicalObj, lastObj: PhysicalObj ) -> felt {
    // Adds a object to array
    // Returns pointer to the object
    // Separate array to hold acceleration for the physical object
    // Returns the pointer to newly allocated array
}

func updatePhysicalObject( object: PhysicalObj, acceleration: Vector ) -> PhysicalObj {
    let newVel = Vector(
        object.velocity.x + acceleration.x,
        object.velocity.y + acceleration.y
    );
    let newPos = Vector(
        object.position.x + object.velocity.x,
        object.position.y + object.velocity.y
    );
    // Create PhysicalObj with newPos and newVel and return
}

func main() {
    
}