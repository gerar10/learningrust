use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

fn car_quality (miles: u32) -> (Age, u32) {
    if miles > 0 {
        return (Age::Used, miles)
    }
    (Age::New, miles)
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
  if car_quality(miles).0 == Age::Used {
      if roof {
          println!("Preparing a used car: {:?}, {}, Hard top, {} miles ", motor, color, miles);
      } else {
          println!("Preparing a used car: {:?}, {}, Convertible, {} miles", motor, color, miles);
      } 
      }
      else {
          if roof {
              println!("Building a new car: {:?}, {}, Hard top, {} miles", motor, color, miles);
          } else {
              println!("biulding a new car: {:?}, {}, Convertible, {} miles", motor, color, miles)
          }
      }

  Car {
      color: color,
      motor: motor,
      roof: roof,
      age: car_quality(miles)
  }
}

fn main() {
  car_factory(String::from("Orange"), Transmission::Manual, true, 0);

  car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

  car_factory(String::from("White"), Transmission::Automatic, true, 3000);

  let mut reviews: HashMap<String, String> = HashMap::new();
  reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
  reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
  reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));
  let book: &str = "Programming in Rust";
  println!("\nReview for \'{}\': {:?}", book, reviews.get(book));


  let obsolete: &str = "Ancient Roman History";
  println!("\n'{}\' removed.", obsolete);
  reviews.remove(obsolete);

  println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));

  prueba();
  contando();
  iterando();
}

fn prueba() {
    loop {
        println!("We loop forever!");
        break;
    }

let mut contador =1;
let stop_loop = loop {
    contador *= 2;
    if contador > 100 {
        break contador
    }
};    
println!("Break the loop at counter = {}", stop_loop)
}

fn contando () {
    let mut counter = 1;
    while counter < 5 {
        println!("We loop a while... {}", counter);
        counter = counter + 1;
    }
}

fn iterando () {
    let big_birds = ["ostrich", "peacock", "stork"];
    for bird in big_birds.iter() {
        println!("The {} is a big bird.", bird);
        }

    for number in 0..5 {
        println!("{}", number * 2);
    }    
}
