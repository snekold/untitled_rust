fn main(){
    let name = "name";
    let mut guess = String::new();
    let name: u32 = 23123;

    const USER_MAX_SCORE: u32 = 1_000_000;
    println!("Info: {}", USER_MAX_SCORE);

    // Tuple (кортеж)
    let user_alex: (i8, bool, f32, char) = (42,true,1.86,'R');
    //user_alex.2 = 2.0; нельзя, потому что не "mut"
    //user_alex.4 = 2.0; нельзя, потому что всего 0 1 2 3 ячеек
    println!("Info: {}", user_alex.0); // i8; 42

    // Array(массив)
    let mut nums : [u8; 5] = [1,5,2,7,3];
    nums[0] = 10;
    println!("Info: {}", nums[0]);

}