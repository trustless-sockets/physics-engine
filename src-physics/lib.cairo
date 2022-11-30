mod structs;
mod functions;

use structs::PhysicalObj;
use functions::newObj;
use functions::applyForce;
use functions::updateObject;

// Returns an array of size n with the values of the Fibonacci sequence.
func main() -> ((felt, felt), (felt, felt)) {
    let ob = newObj( 0xf00, 0xc1d, 200, 500 );
    let ob = applyForce( ob, (5,0) );
    let ob = applyForce( ob, (0,11) );
    let (x, y) = ob.position;
    let (vx, vy) = ob.velocity;
    let ob = updateObject( ob );
    (ob.velocity,
    ob.position)
}

