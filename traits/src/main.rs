trait Vehicle: Paint { //Paint is a supertrait of Vehicle
    fn park(&self);
    fn get_default_color() -> String 
    {
        "black".to_owned()
    }
}


trait Paint {
    fn paint(&self, color: String) {
        println!("painting object: {}", color);
    }
}

struct VehicleInfo {
    make: String,
    model: String,
    year: u16
}

struct Car {
    info: VehicleInfo
}
impl Park for Car {
    fn park(&self) {
        println!("parking car");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehicleInfo
}


impl Truck {
    fn unload(&self) {
        println!("Unloading truck");
    }
    
}

impl Park for Truck {
    fn park(&self) {
        println!("parking truck");
    }
}
 impl Paint for Truck {}

 struct House {}

 impl Paint for House {
    fn paint(&self, color: String) {
        println!("painting house: {}", color);
    }
}


fn main() {
    let car = Car{
        info: VehicleInfo{
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    };
    let house = House {};
    let object = create_paintable_object(true);

    let paintable_objects:Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&car);
    paint_red(&house);
    paint_red(object.as_ref());

    paint_vehicle_red(&car);
    paint_vehicle_red(&house); 
    paint_vehicle_red(&object);
    //house and object don't implement the Park trait
}

fn paint_red(object: &dyn Paint) { 
    object.paint("red".to_owned());

}

fn paint_red2(object: & impl Paint) { // object must be something implement in Paint
    object.paint("red".to_owned());

}

/*fn paint_vehicle_red<T>(object: &T) where T: Paint + Vehicle { //use it when you have multiple trait bounds
    object.paint("red".to_owned());

}*/
fn paint_vehicle_red<T>(object: &T) where T: Vehicle{//anything implements Vehicle also implement Paint
    object.paint("red".to_owned());
}


fn create_paintable_object(vehicle: bool) -> Box<dyn Paint>{ //box smart pointer allocated on the heap
    if vehicle {
        Box::new(Car {
           info: VehicleInfo{
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 1995
           }
        })
    } else {
        Box::new(House {})
    }
    
}
//static dispatch: when the compiler knows which method to call at compile time
//dyn dispatch: the compiler cant figure out which concrete method to call at compile time Pros: flexibility Disadvantage: Runtime performance cost

