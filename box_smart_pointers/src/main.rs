trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");

    }
}
struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Container{
    name:String,
    child: Box<Container> //now child has the same size as pointer Box
}

impl UIComponent for Container {}

fn main() {
    let button_a = Button { text: "button a".to_owned() };
    let button_b = Box::new(Button { text: "button b".to_owned()});

    let button_c = button_a;
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d
    ];
}

//smart pointers avoid to copy big amount of data while transfering ownership
