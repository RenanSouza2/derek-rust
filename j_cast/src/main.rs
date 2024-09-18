fn main() {
    enum Day {
        Monday,
        _Tuesday,
        _Wedneday,
        _Thursday,
        _Friday,
        _Saturday,
        _Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::_Saturday | Day::_Sunday => true,
                _ => false,
            }
        }
    }
    let today = Day::Monday;
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::_Tuesday => println!("Donut day"),
        Day::_Wedneday => println!("Hump day"),
        Day::_Thursday => println!("Pay day"),
        Day::_Friday => println!("Almost Weekend"),
        Day::_Saturday => println!("Weekend"),
        Day::_Sunday => println!("Weekend"),
    }

    println!("Is today the weekend : {}", today.is_weekend());
}
