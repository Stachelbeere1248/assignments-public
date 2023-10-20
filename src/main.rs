fn main() {
    let valid_states: [bool; 2] = [true, false];

    for x in valid_states {
        for y in valid_states {
            println!("X:{}, Y:{}, 1a(XY):{}, 1b(XY):{}, 3a(XY):{}", x, y, aa(x,y), ab(x, y), ca(x,y))
        }
    }
}
//Formel der Aufgabe 1. a)
fn aa(x: bool, y: bool) -> bool {
    implies(implies(x,y&&!y),!x)
}
//Formel der Aufgabe 1. b)
fn ab(x: bool, y: bool) -> bool {
    implies(x && y , !x && !y)
}
//Formel "(nicht A)und(nicht B)" zum Vergleich in der Aufgabe 3. a)
fn ca(x:bool, y:bool) -> bool {
    //star(x,y)==(!x&&!y)
    !x&&!y
}
//fn star(x:bool, y: bool) -> bool {
//    !(x||y)
//}
fn implies(b1: bool, b2: bool) -> bool {
    !b1||b2
}