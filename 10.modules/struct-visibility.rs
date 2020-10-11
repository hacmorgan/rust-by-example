/*
Structs have an extra level of visibility in their fields. The visibility 
defaults to private, and can be overridden with the pub modifier. This 
visibility only matters when a struct is accessed from outside the module where
it is defined, and has the goal of hiding information (encapsulation).
 */

mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl< T : std::fmt::Debug > ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }

        // A homemade function that will show the contents of the box
        // n.b. T had to be refined to types that implement std::fmt::Debug
        // in order to make this work
        pub fn show( self )
        {
            println!( "My contents are: {:?}", self.contents );
        }
    }
}

fn main() {
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    // let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");
    _closed_box.show()

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    // println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}
