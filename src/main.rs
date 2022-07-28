struct Rect {
    width: u32,
    height: u32
}

fn main() {
    // using single vars
    let width = 12;
    let height = 10;

    println!("[using single vars] Area is: {}", calculate_area(width, height));

    // using tuples
    let rect = (12, 10);

    println!("[using tuples] Area of rect is: {}", calculate_area2(rect));

    // using struct 
    let rect2 = Rect {
        width: 12,
        height: 10
    };

    println!("[using struct] Area of rect is: {}", calculate_area3(&rect2));
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
} 

fn calculate_area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
} 

fn calculate_area3(rect: &Rect) -> u32 {
    rect.width * rect.height
}  