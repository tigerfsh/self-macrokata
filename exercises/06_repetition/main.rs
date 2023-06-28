////////// DO NOT CHANGE BELOW HERE /////////
fn print_success() {
    println!("Yay, the if statement worked.");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `if_any!()` macro.
macro_rules! if_any {
    ($($flag:expr),+; $block:block) => {
        // flag1 || flag2 || flag3
        {
            if $($flag)||+ {
                $block
            }
        }
        


    };
}
////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    if_any!(false, 0 == 1, true; {
        print_success();
    })
}

/*

macro_rules! listing_literals {
    ($(the $my_literal:literal)and+) => {
        {
            let mut my_vec = Vec::new();
            $(my_vec.push($my_literal);)+ // => $(my_vec.push($my_literal));+;
            /*
            my_vec.push(s1);
            my_vec.push(s2);
            my_vec.push(s3);

             */
            my_vec
        }
    }
}

 */
